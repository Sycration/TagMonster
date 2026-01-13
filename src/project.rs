use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub name: String,
    pub top_folder_id: usize,
    pub box_url: String,
    pub sheets_url: String,
    pub sheet_id: i32
}
