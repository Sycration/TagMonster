use crate::{box_source::BoxSource, local_source::LocalSource, project_page::{ChildCounts, InternalType, Node}};
use r#box::apis::{
    configuration::Configuration,
    downloads_api::{GetFilesIdContentParams, get_files_id_content},
    folders_api::{GetFoldersIdItemsParams, GetFoldersIdParams},
};
use pure_magic::MagicDb;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tracing::*;



#[derive(Debug, Clone, Default)]
pub struct RequiredData {
    pub box_conf: Option<Configuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Box(BoxSource),
    Local(LocalSource)
}

impl Source {
    pub fn name(&self) -> String {
        match self {
            Source::Box(b) => b.name(),
            Source::Local(l) => l.name(),
        }
    }
    pub fn get_top_folder_id(&self) -> String {
        match self {
            Source::Box(b) => b.get_top_folder_id(),
            Source::Local(l) => l.get_top_folder_id(),
        }
    }
    pub fn get_top_folder_url(&self) -> String {
        match self {
            Source::Box(b) => b.get_top_folder_url(),
            Source::Local(l) => l.get_top_folder_url(),
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
            Source::Local(l) => l.list_contents(req_data, folder_id, flat).await,
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
            Source::Local(l) => l.get_info(req_data, item_id, file_type).await,
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
            Source::Local(l) => l.get_file_type(req_data, file, db).await,
        }
    }
}

