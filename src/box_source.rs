use std::io::Cursor;

use r#box::apis::{
    configuration::Configuration,
    downloads_api::{GetFilesIdContentParams, get_files_id_content},
    folders_api::{GetFoldersIdItemsParams, GetFoldersIdParams},
    shared_links_folders_api::GetSharedItemsFoldersParams,
};
use iced::debug;
use pure_magic::MagicDb;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tracing::{debug, error, warn};
use url::Url;

use crate::{
    project_page::{ChildCounts, InternalType, Node},
    source::RequiredData,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxSource {
    pub top_folder_id: String,
    pub box_url: String,
    hostname: String,
    name: String,
    share_id: Option<String>,
}

impl BoxSource {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn new(url: &str, req_data: &RequiredData) -> anyhow::Result<Self> {
        let mut share_id = None;
        let mut url: Url = url.parse()?;
        url.set_query(None);
        let configuration = match &req_data.box_conf {
            Some(c) => c,
            None => {
                tracing::warn!("Not logged in to Box");
                anyhow::bail!("Not logged in to Box");
            }
        };
        let hostname = url.origin().ascii_serialization();
        let box_id = if url.path().contains("/s/") {
            if url.path().contains("/folder/") {
                // it does contain a path
                let path_segs = url.path_segments().unwrap().collect::<Vec<_>>();

                // it does have this string in the path
                let target = path_segs.iter().position(|seg| *seg == "folder").unwrap();

                let i_share_id = path_segs
                    .get(target - 1)
                    .ok_or(anyhow::format_err!("No share ID"))?;
                let subfolder_id = path_segs
                    .get(target + 1)
                    .ok_or(anyhow::format_err!("No subfolder ID"))?;

                let mut truncated_url = url.clone();

                let path_up_to_folder = &path_segs[0..=target - 1];
                truncated_url.set_path(&path_up_to_folder.join("/"));

                let _shared_root = r#box::apis::shared_links_folders_api::get_shared_items_folders(
                    configuration,
                    GetSharedItemsFoldersParams {
                        boxapi: format!("shared_link={}", truncated_url),
                        if_none_match: None,
                        fields: None,
                    },
                )
                .await?;

                share_id = Some(i_share_id.to_string());

                subfolder_id.to_string()
            } else {
                share_id = url.path_segments().map(|s|s.last()).flatten().map(|s|s.to_string());

                let folder = r#box::apis::shared_links_folders_api::get_shared_items_folders(
                    configuration,
                    GetSharedItemsFoldersParams {
                        boxapi: share_id
                            .as_ref()
                            .map(|s| dbg!(format!("shared_link={}/s/{s}", &hostname)))
                            
                            .unwrap_or_default(),
                        if_none_match: None,
                        fields: None,
                    },
                )
                .await?;
                folder.id
            }
        } else {
            url.path_segments()
                .ok_or(anyhow::format_err!("Invalid URL"))?
                .last()
                .ok_or(anyhow::format_err!("Invalid URL"))?
                .to_string()
        };

        let box_id: usize = match box_id.parse() {
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
                boxapi: share_id
                    .as_ref()
                    .map(|s| format!("shared_link={}/s/{s}", &hostname)),
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
            share_id: share_id,
            hostname: hostname.to_string(),
        })
    }

    pub async fn get_info(
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
                        boxapi: self
                            .share_id
                            .as_ref()
                            .map(|s| format!("shared_link={}/s/{s}", &self.hostname)),
                        x_rep_hints: None,
                    },
                )
                .await?;
                Ok(Node {
                    name: file.name.unwrap_or_else(|| "UNNAMED FILE".to_string()),
                    link: format!(
                            "{}{}/file/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &file.id
                        ),
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
                        boxapi: self
                            .share_id
                            .as_ref()
                            .map(|s| format!("shared_link={}/s/{s}", &self.hostname)),
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
                            "{}{}/folder/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &folder.id
                        ),
                    id: folder.id,
                    idx: 0,
                    file_type: InternalType::Folder,
                    children: None,
                    child_counts: None,
                })
            }
            InternalType::Link => {
                let file = r#box::apis::web_links_api::get_web_links_id(
                    configuration,
                    r#box::apis::web_links_api::GetWebLinksIdParams {
                        web_link_id: item_id.to_string(),
                        boxapi: self
                            .share_id
                            .as_ref()
                            .map(|s| format!("shared_link={}/s/{s}", &self.hostname)),
                    },
                )
                .await?;
                Ok(Node {
                    name: file.name.unwrap_or_else(|| "UNNAMED LINK".to_string()),
                    link: format!(
                            "{}{}/web_link/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &file.id
                        ),
                    id: file.id,
                    idx: 0,
                    file_type: InternalType::Link,
                    children: None,
                    child_counts: None,
                })
            }
        }
    }

    pub async fn list_folder_contents_inner(
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
                boxapi: self
                    .share_id
                    .as_ref()
                    .map(|s| format!("shared_link={}/s/{s}", &self.hostname)),
                sort: None,
                direction: None,
            },
        )
        .await?
        .entries
        .unwrap_or_default();
        Ok(entries)
    }

    pub fn get_top_folder_id(&self) -> String {
        self.top_folder_id.clone()
    }

    pub fn get_top_folder_url(&self) -> String {
        self.box_url.clone()
    }

    pub async fn list_contents(
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
                        link: format!(
                            "{}{}/file/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &f.id
                        ),
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
                        link: format!(
                            "{}{}/folder/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &f.id
                        ),
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
                        link: format!(
                            "{}{}/web_link/{}",
                            self.hostname.trim_end_matches('/'),
                            self.share_id
                                .as_ref()
                                .map(|s| format!("/s/{s}"))
                                .unwrap_or_default(),
                            &f.id
                        ),
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

    pub async fn get_file_type(
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
