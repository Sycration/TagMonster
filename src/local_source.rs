use std::{
    ffi::OsStr,
    fs::DirEntry,
    io::Cursor,
    path::{Path, PathBuf},
    pin::Pin,
};

use r#box::apis::{
    configuration::Configuration,
    downloads_api::{GetFilesIdContentParams, get_files_id_content},
    folders_api::{GetFoldersIdItemsParams, GetFoldersIdParams},
};
use google_sheets4::yup_oauth2::error;
use pure_magic::MagicDb;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufRead, BufReader, ReadBuf};
use tokio_stream::StreamExt;
use tracing::{debug, error, warn};

use crate::{
    project_page::{ChildCounts, InternalType, Node},
    source::RequiredData,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalSource {
    pub top_folder_path: PathBuf,
}

impl LocalSource {
    pub fn name(&self) -> String {
        self.top_folder_path
            .file_name()
            .unwrap_or(OsStr::new("UNNAMED FOLDER"))
            .to_string_lossy()
            .into_owned()
    }

    pub async fn new(url: &str, req_data: &RequiredData) -> anyhow::Result<Self> {
        let path = tokio::fs::canonicalize(url).await.inspect_err(|e| {
            error!("Failed to canonicalize path {url}: {}", e);
        })?;

        if path.is_dir() {
            Ok(LocalSource {
                top_folder_path: path,
            })
        } else {
            error!("Provided path is not a directory");
            anyhow::bail!("Provided path is not a directory");
        }
    }

    pub async fn get_info(
        &self,
        _req_data: &RequiredData,
        item_id: &str,
        file_type: InternalType,
    ) -> anyhow::Result<Node> {
        match file_type {
            InternalType::File => {
                let link = match url::Url::from_file_path(item_id).map(|u| u.to_string()) {
                    Ok(l) => l,
                    Err(_) => {
                        error!("Failed to create file URL for {}", item_id);
                        anyhow::bail!("Failed to create file URL for {}", item_id);
                    }
                };

                Ok(Node {
                    name: PathBuf::from(item_id)
                        .file_name()
                        .ok_or_else(|| {
                            anyhow::anyhow!("Failed to extract file name from path: {}", item_id)
                        })?
                        .to_string_lossy()
                        .into_owned(),
                    link: link,
                    id: item_id.to_string(),
                    idx: 0,
                    file_type: InternalType::File,
                    children: None,
                    child_counts: None,
                })
            }
            InternalType::Folder => {
                let link = match url::Url::from_directory_path(item_id).map(|u| u.to_string()) {
                    Ok(l) => l,
                    Err(_) => {
                        error!("Failed to create directory URL for {}", item_id);
                        anyhow::bail!("Failed to create directory URL for {}", item_id);
                    }
                };

                let contents = tokio::fs::read_dir(item_id).await.inspect_err(|e| {
                    error!("Failed to read directory {}: {}", item_id, e);
                })?;
                let mut folder_count = 0;
                let mut file_count = 0;

                tokio::pin!(contents);

                while let Some(entry) = contents.next_entry().await.inspect_err(|e| {
                    error!("Failed to read directory entry in {}: {}", item_id, e);
                })? {
                    let metadata = entry.metadata().await.inspect_err(|e| {
                        error!("Failed to get metadata for entry {:?}: {}", entry.path(), e);
                    })?;
                    if metadata.is_dir() {
                        folder_count += 1;
                    } else if metadata.is_file() {
                        file_count += 1;
                    }
                }

                Ok(Node {
                    name: PathBuf::from(item_id)
                        .file_name()
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "Failed to extract directory name from path: {}",
                                item_id
                            )
                        })?
                        .to_string_lossy()
                        .into_owned(),
                    link: link,
                    id: item_id.to_string(),
                    idx: 0,
                    file_type: InternalType::Folder,
                    children: None,
                    child_counts: Some(ChildCounts {
                        folder_count,
                        file_count,
                    }),
                })
            }
            _ => {
                error!("It should be impossible to have a web link stored on the local filesystem");
                anyhow::bail!(
                    "It should be impossible to have a web link stored on the local filesystem"
                );
            }
        }
    }

    pub fn get_top_folder_id(&self) -> String {
        self.top_folder_path.to_string_lossy().into_owned()
    }

    pub fn get_top_folder_url(&self) -> String {
        url::Url::from_directory_path(self.top_folder_path.clone())
            .map(|u| u.to_string())
            .unwrap_or_else(|_| {
                warn!("Failed to create directory URL for top folder");
                "file://INVALID_PATH".to_string()
            })
    }

    pub async fn list_contents(
        &self,
        req_data: &RequiredData,
        folder_id: &str,
        flat: bool,
    ) -> anyhow::Result<Vec<Node>> {
        let mut read_dir = tokio::fs::read_dir(folder_id).await.inspect_err(|e| {
            error!("Failed to read directory {}: {}", folder_id, e);
        })?;

        let mut entries = Vec::new();
        while let Some(entry) = read_dir.next_entry().await.inspect_err(|e| {
            error!("Failed to read directory entry in {}: {}", folder_id, e);
        })? {
            let metadata = entry.metadata().await.inspect_err(|e| {
                error!("Failed to get metadata for entry {:?}: {}", entry.path(), e);
            })?;
            if metadata.is_file() {
                entries.push(entry);
            } else if metadata.is_dir() {
                entries.push(entry);
            }
        }

        let mut nodes: Vec<Node> = Vec::with_capacity(entries.len());

        let mut folder_idx = 0;
        let mut file_idx = 0;

        for entry in entries.into_iter() {
            // we know it's either a file or folder from above, so unwrap is safe
            if entry.metadata().await.unwrap().is_file() {
                nodes.push(Node {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    link: url::Url::from_file_path(entry.path())
                        .map(|u| u.to_string())
                        .unwrap_or_else(|_| {
                            warn!("Failed to create file URL for {:?}", entry.path());
                            "file://INVALID_PATH".to_string()
                        }),
                    id: entry.path().to_string_lossy().into_owned(),
                    idx: file_idx,
                    file_type: InternalType::File,
                    children: None,
                    child_counts: None,
                });
                file_idx += 1;
            } else {
                // is folder
                let (children, child_counts) = if flat {
                    let child_entries = self.list_folder_contents_inner(&entry.path()).await?;

                    let mut counts = ChildCounts {
                        file_count: 0,
                        folder_count: 0,
                    };
                    for entry in child_entries.into_iter() {
                        if let Ok(metadata) = entry.metadata().await {
                            if metadata.is_dir() {
                                counts.folder_count += 1;
                            } else if metadata.is_file() {
                                counts.file_count += 1;
                            }
                        }
                    }
                    (None, Some(counts))
                } else {
                    let child_nodes = Box::pin(self.list_contents(
                        req_data,
                        &entry.path().to_string_lossy(),
                        false,
                    ))
                    .await?;
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
                    name: entry.file_name().to_string_lossy().into_owned(),
                    link: url::Url::from_directory_path(entry.path())
                        .map(|u| u.to_string())
                        .unwrap_or_else(|_| {
                            warn!("Failed to create directory URL for {:?}", entry.path());
                            "file://INVALID_PATH".to_string()
                        }),
                    id: entry.path().to_string_lossy().into_owned(),
                    idx: folder_idx,
                    file_type: InternalType::Folder,
                    children: children,
                    child_counts,
                });
                folder_idx += 1;
            }
        }

        Ok(nodes)
    }

    pub async fn list_folder_contents_inner(
        &self,
        folder_id: &Path,
    ) -> anyhow::Result<Vec<tokio::fs::DirEntry>> {
        let mut readdir = tokio::fs::read_dir(folder_id).await.inspect_err(|e| {
            error!("Failed to read directory {:?}: {}", folder_id, e);
        })?;

        let mut entries = Vec::new();
        while let Some(entry) = readdir.next_entry().await.inspect_err(|e| {
            error!("Failed to read directory entry in {:?}: {}", folder_id, e);
        })? {
            if entry
                .metadata()
                .await
                .inspect_err(|e| {
                    error!("Failed to get metadata for entry {:?}: {}", entry.path(), e);
                })?
                .is_dir()
            {
                entries.push(entry);
                continue;
            }
        }
        Ok(entries)
    }

    pub async fn get_file_type(
        &self,
        _req_data: &RequiredData,
        file: &Node,
        db: &MagicDb,
    ) -> anyhow::Result<String> {
        let f = std::fs::File::open(&file.id)?;
        let mut reader = std::io::BufReader::new(f);
        Ok(db.best_magic(&mut reader)?.message())
    }
}
