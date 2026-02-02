use anyhow::Ok;
use axum::extract::path;
use tokio::{fs::File, io::{AsyncWriteExt, BufWriter}};

use crate::{
    CONFIG_DIR,
    export::{add_top_folder_node, flatten_node},
    persist,
    project::Project,
    project_page::{InternalType, Node},
    source::RequiredData,
};

pub async fn make_csv(
    req_data: &RequiredData,
    project: Project,
    entries: Vec<Node>,
    mut file: File,
    path: &str,
) -> anyhow::Result<()> {
    let tree = add_top_folder_node(req_data, &project, entries).await?;

    let mut flat: Vec<Node> = Vec::new();
    flatten_node(&tree, &mut flat);
    tracing::info!(
        "Flattened tree for {}: {} entries",
        project.name,
        flat.len()
    );

    let mut writer = csv_async::AsyncWriter::from_writer(&mut file);


    let db = magic_db::load()?;

    // writer.write_all(b"").await?;
    writer.write_record(["counts","loose marker","folder name","file name","URL","file type tag"]).await?;

    for node in flat.iter() {
        let counts = if let Some(counts) = &node.child_counts {
            format!(
                "({} folders) ({} files)",
                counts.folder_count, counts.file_count
            )
        } else {
            "".to_string()
        };
        let loose_marker = if node.file_type != InternalType::Folder && node.idx == 0 {
            "Loose files:"
        } else {
            ""
        };
        let folder_name = if node.file_type == InternalType::Folder {
            &node.name
        } else {
            ""
        };
        let file_name = if node.file_type != InternalType::Folder {
            &node.name
        } else {
            ""
        };
        let url = &node.link;
        let file_type_tag = match node.file_type {
            InternalType::File => project
                .source
                .get_file_type(req_data, &node, &db)
                .await.unwrap_or_else(|e|{
                    tracing::warn!("Error getting file type for {}: {}", node.name, e);
                    "Unknown".to_string()
                }),
            InternalType::Folder => "".to_string(),
            InternalType::Link => "Web link".to_string(),
        };

        writer.write_record([
            &counts,
            loose_marker,
            folder_name,
            file_name,
            url,
            &file_type_tag,
        ]).await?;

        tracing::debug!(
            "Wrote CSV record: {}, {}, {}, {}, {}, {}",
            counts,
            loose_marker,
            folder_name,
            file_name,
            url,
            file_type_tag
        );
    }
        

    writer.flush().await?;

        tracing::info!(
        "Done writing data to {}",
        path
    );

    Ok(())
}
