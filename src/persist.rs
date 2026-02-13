use std::path::Path;

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use tokio::{fs::{self, OpenOptions}, io::AsyncWriteExt};

pub async fn persist<T: Serialize + for<'a> Deserialize<'a>>(
    data: &T,
    config_dir: &Path,
    name: &str,
) -> anyhow::Result<()> {
    let file_path = config_dir.join(format!("{name}.json"));
    let temp_path = config_dir.join(format!(".{name}.json.tmp"));
    
    let mut temp_file = fs::File::create(&temp_path).await?;
    let token_json = serde_json::to_string_pretty(&data)?;
    temp_file.write_all(token_json.as_bytes()).await?;
    temp_file.sync_all().await?;
    
    fs::rename(&temp_path, &file_path).await?;
    
    Ok(())
}

pub async fn retrieve<T: Default + Serialize + for<'a> Deserialize<'a>>(
    config_dir: &Path,
    name: &str,
) -> Result<T> {
    tokio::fs::read_to_string(config_dir.join(format!("{name}.json")))
        .await
        .map_err(anyhow::Error::from)
        .and_then(|json| serde_json::from_str::<T>(&json).map_err(anyhow::Error::from))
}

#[allow(unused)]
pub fn retrieve_sync<T: Default + Serialize + for<'a> Deserialize<'a>>(
    config_dir: &Path,
    name: &str,
) -> Result<T> {
    std::fs::read_to_string(config_dir.join(format!("{name}.json")))
        .map_err(anyhow::Error::from)
        .and_then(|json| serde_json::from_str::<T>(&json).map_err(anyhow::Error::from))
}
