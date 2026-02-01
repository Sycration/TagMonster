use crate::project_page::{ChildCounts, InternalType, Node};
use r#box::apis::{
    configuration::Configuration,
    downloads_api::{GetFilesIdContentParams, get_files_id_content},
    folders_api::{GetFoldersIdItemsParams, GetFoldersIdParams},
};
use pure_magic::MagicDb;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tracing::*;

use tokio_stream::StreamExt;

#[derive(Debug, Clone, Default)]
pub struct RequiredData {
    pub box_conf: Option<Configuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Box(BoxSource),
}

impl Source {
    pub fn name(&self) -> &str {
        match self {
            Source::Box(b) => b.name(),
        }
    }
    pub fn get_top_folder_id(&self) -> &str {
        match self {
            Source::Box(b) => b.get_top_folder_id(),
        }
    }
    pub fn get_top_folder_url(&self) -> &str {
        match self {
            Source::Box(b) => b.get_top_folder_url(),
        }
    }
    pub async fn list_contents(
        &self,
        req_data: &RequiredData,
        folder_id: &str,
        flat: bool,
    ) -> anyhow::Result<Vec<Node>> {
        match self {
            Source::Box(b) => b.list_contents(req_data, folder_id, flat).await,
        }
    }
    pub async fn get_info(
        &self,
        req_data: &RequiredData,
        item_id: &str,
        file_type: InternalType,
    ) -> anyhow::Result<Node> {
        match self {
            Source::Box(b) => b.get_info(req_data, item_id, file_type).await,
        }
    }
    pub async fn get_file_type(
        &self,
        req_data: &RequiredData,
        file: &Node,
        db: &MagicDb,
    ) -> anyhow::Result<String> {
        match self {
            Source::Box(b) => b.get_file_type(req_data, file, db).await,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxSource {
    pub top_folder_id: String,
    pub box_url: String,
    hostname: String,
    name: String,
}

impl BoxSource {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn new(url: &str, req_data: &RequiredData) -> anyhow::Result<Self> {
        let configuration = match &req_data.box_conf {
            Some(c) => c,
            None => {
                tracing::warn!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        let box_id = url.split('/').last().unwrap_or_default().parse();
        let hostname = url.split('/').take(3).collect::<Vec<&str>>().join("/");
        let box_id: usize = match box_id {
            Ok(id) => id,
            Err(e) => {
                tracing::warn!("Invalid Box URL {}: {}", url, e);
                anyhow::bail!("Invalid Box URL {}: {}", url, e);
            }
        };

        let folder = r#box::apis::folders_api::get_folders_id(
            configuration,
            GetFoldersIdParams {
                folder_id: box_id.to_string(),
                fields: None,
                if_none_match: None,
                boxapi: None,
                sort: None,
                direction: None,
                offset: None,
                limit: None,
            },
        )
        .await?;

        Ok(BoxSource {
            name: folder.name.unwrap_or_else(|| folder.id.clone()),
            top_folder_id: folder.id,
            box_url: url.to_string(),
            hostname: hostname,
        })
    }

    async fn get_info(
        &self,
        req_data: &RequiredData,
        item_id: &str,
        file_type: InternalType,
    ) -> anyhow::Result<Node> {
        let configuration = match &req_data.box_conf {
            Some(c) => c,
            None => {
                tracing::warn!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        match file_type {
            InternalType::File => {
                let file = r#box::apis::files_api::get_files_id(
                    configuration,
                    r#box::apis::files_api::GetFilesIdParams {
                        file_id: item_id.to_string(),
                        fields: None,
                        if_none_match: None,
                        boxapi: None,
                        x_rep_hints: None,
                    },
                )
                .await?;
                Ok(Node {
                    name: file.name.unwrap_or_else(|| "UNNAMED FILE".to_string()),
                    link: format!("{}/file/{}", self.hostname.trim_end_matches('/'), &file.id),
                    id: file.id,
                    idx: 0,
                    file_type: InternalType::File,
                    children: None,
                    child_counts: None,
                })
            }
            InternalType::Folder => {
                let folder = r#box::apis::folders_api::get_folders_id(
                    &configuration,
                    r#box::apis::folders_api::GetFoldersIdParams {
                        folder_id: item_id.to_string(),
                        fields: Some(vec!["id".to_string(), "name".to_string()]),
                        if_none_match: None,
                        boxapi: None,
                        sort: None,
                        direction: None,
                        offset: None,
                        limit: None,
                    },
                )
                .await?;
                Ok(Node {
                    name: folder.name.unwrap_or_else(|| "UNNAMED FOLDER".to_string()),
                    link: format!(
                        "{}/folder/{}",
                        self.hostname.trim_end_matches('/'),
                        &folder.id
                    ),
                    id: folder.id,
                    idx: 0,
                    file_type: InternalType::Folder,
                    children: None,
                    child_counts: None,
                })
            }
            _ => {
                anyhow::bail!("Unsupported item type for get_info");
            }
        }
    }

    async fn list_folder_contents_inner(
        &self,
        configuration: &Configuration,
        folder_id: &str,
    ) -> anyhow::Result<Vec<r#box::models::Item>> {
        let entries = r#box::apis::folders_api::get_folders_id_items(
            configuration,
            GetFoldersIdItemsParams {
                folder_id: folder_id.to_owned(),
                fields: None,
                usemarker: None,
                marker: None,
                offset: None,
                limit: Some(500),
                boxapi: None,
                sort: None,
                direction: None,
            },
        )
        .await?
        .entries
        .unwrap_or_default();
        Ok(entries)
    }

    fn get_top_folder_id(&self) -> &str {
        &self.top_folder_id
    }

    fn get_top_folder_url(&self) -> &str {
        &self.box_url
    }

    async fn list_contents(
        &self,
        req_data: &RequiredData,
        folder_id: &str,
        flat: bool,
    ) -> anyhow::Result<Vec<Node>> {
        let configuration = match &req_data.box_conf {
            Some(c) => c,
            None => {
                tracing::warn!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        let entries = self
            .list_folder_contents_inner(configuration, folder_id)
            .await?;

        let mut nodes: Vec<Node> = Vec::with_capacity(entries.len());

        let mut folder_idx = 0;
        let mut file_idx = 0;

        for entry in entries.into_iter() {
            match entry {
                r#box::models::Item::FileFull(f) => {
                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED FILE".to_string()),
                        link: format!("{}/file/{}", self.hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: file_idx,
                        file_type: InternalType::File,
                        children: None,
                        child_counts: None,
                    });
                    file_idx += 1;
                }
                r#box::models::Item::FolderMini(f) => {
                    let (children, child_counts) = if flat {
                        let child_entries = self
                            .list_folder_contents_inner(configuration, folder_id)
                            .await?;
                        let counts = child_entries.iter().fold(
                            ChildCounts {
                                file_count: 0,
                                folder_count: 0,
                            },
                            |mut acc, entry| {
                                match entry {
                                    r#box::models::Item::FolderMini(_) => acc.folder_count += 1,
                                    _ => acc.file_count += 1,
                                }
                                acc
                            },
                        );
                        (None, Some(counts))
                    } else {
                        let child_nodes =
                            Box::pin(self.list_contents(req_data, &f.id, false)).await?;
                        let folder_count = child_nodes
                            .iter()
                            .filter(|n| n.file_type == InternalType::Folder)
                            .count();
                        let file_count = child_nodes
                            .iter()
                            .filter(|n| n.file_type != InternalType::Folder)
                            .count();
                        (
                            Some(child_nodes),
                            Some(ChildCounts {
                                file_count,
                                folder_count,
                            }),
                        )
                    };

                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED FOLDER".to_string()),
                        link: format!("{}/folder/{}", self.hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: folder_idx,
                        file_type: InternalType::Folder,
                        children: children,
                        child_counts,
                    });
                    folder_idx += 1;
                }
                r#box::models::Item::WebLink(f) => {
                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED LINK".to_string()),
                        link: format!("{}/web_link/{}", self.hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: file_idx,
                        file_type: InternalType::Link,
                        children: None,
                        child_counts: None,
                    });
                    file_idx += 1;
                }
            };
        }

        Ok(nodes)
    }

    async fn get_file_type(
        &self,
        req_data: &RequiredData,
        file: &Node,
        db: &MagicDb,
    ) -> anyhow::Result<String> {
        let configuration = match &req_data.box_conf {
            Some(c) => c,
            None => {
                tracing::warn!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        let resp = get_files_id_content(
            configuration,
            GetFilesIdContentParams {
                file_id: file.id.clone(),
                range: Some(format!("bytes=0-{}", (100 * 1024))),
                boxapi: None,
                version: None,
                access_token: None,
            },
        )
        .await
        .map(|f| f.url().to_owned());
        let url = match resp {
            Ok(url) => url,
            Err(e) => {
                error!("Failed to get file download URL: {}", e);
                anyhow::bail!("Failed to get file download URL: {}", e);
            }
        };
        let token = match configuration.oauth_access_token.clone() {
            Some(t) => t,
            None => {
                error!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        let resp = match configuration
            .client
            .get(url.to_owned())
            .bearer_auth(&token)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to download file {}: {}", file.name, e.to_string());
                anyhow::bail!("Failed to download file {}: {}", file.name, e.to_string());
            }
        };
        let mut buf = if let Some(l) = resp.content_length() {
            Vec::with_capacity(l.try_into().unwrap_or(usize::MAX)) // We might be on 32 bit who knows
        } else {
            Vec::new()
        };
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to download file {}: {}", &file.name, e);
                    anyhow::bail!("Failed to download file {}: {}", file.name, e);
                }
            };
            buf.extend_from_slice(&chunk);
        }
        debug!("downloaded file {}", &file.name);
        let mut cursor = Cursor::new(buf);
        let detected = match db.best_magic(&mut cursor) {
            Ok(result) => {
                // pick the first sensible result if present
                result.message()
            }
            Err(_) => {
                warn!("Failed to analyze file {}", &file.name);
                "Unknown".to_string()
            }
        };
        Ok(detected)
    }
}
