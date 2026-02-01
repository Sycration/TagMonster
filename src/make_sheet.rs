use std::io::Cursor;

use r#box::apis::downloads_api::{GetFilesIdContentParams, get_files_id_content};
use google_sheets4::{FieldMask, api::BatchUpdateSpreadsheetRequest};
use iced::{Task, futures::future::join_all};
use pure_magic::MagicDb;
use reqwest::Client;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};

use crate::{
    BATCH_SIZE, CONFIG_DIR, Message, SheetsHub, State, TEMPLATE_ID, export::flatten_node, persist, project::Project, project_page::{InternalType, Node}, source::RequiredData
};

pub async fn make_sheet(req_data: &RequiredData, hub: Option<SheetsHub>, project: Project, entries: Vec<Node>, spreadsheet_id: &str) -> anyhow::Result<()> {
            let hub = if let Some(hub) = hub {
                hub
            } else {
                tracing::error!("Not logged in to Google API");
                anyhow::bail!("Not logged in to Google API");
            };

            let mut tree = project.source.get_info(req_data, project.source.get_top_folder_id(), InternalType::Folder).await?;
            tree.child_counts = Some(Default::default());
            for child in entries.iter() {
                if child.file_type == InternalType::Folder {
                    tree.child_counts.get_or_insert(Default::default()).folder_count += 1;
                } else {
                    tree.child_counts.get_or_insert(Default::default()).file_count += 1;
                }
            }
            tree.children = Some(entries);
            
            let mut flat: Vec<Node> = Vec::new();
            flatten_node(&tree, &mut flat);
            tracing::info!(
                "Flattened tree for {}: {} entries",
                project.name,
                flat.len()
            );
            // persist flattened list for later use
            let _ = persist::persist(
                &flat,
                &CONFIG_DIR.join("projects").join(&project.name),
                &(project.name.clone() + "_flat"),
            )
            .await;
            let copy_req = google_sheets4::api::CopySheetToAnotherSpreadsheetRequest {
                destination_spreadsheet_id: Some(spreadsheet_id.to_string()),
            };
            match hub
                .spreadsheets()
                .sheets_copy_to(copy_req, TEMPLATE_ID, 0)
                .doit()
                .await
            {
                Ok((_resp, props)) => {
                    setup_sheet_basics(&project, tree, &hub, &spreadsheet_id, props).await;
                    tokio::join!(
                        create_filetype_tags(&project, hub.clone(), &req_data, &flat, &spreadsheet_id),
                        create_folder_names(&project, hub.clone(), &flat, &spreadsheet_id),
                        create_file_names(&project, hub.clone(), &flat, &spreadsheet_id),
                    );
                    info!("Done making sheet");
                }
                Err(e) => {
                    tracing::error!("Failed to copy template sheet: {}", e);
                }
            }
            anyhow::Ok(())
}


// Setup basic sheet contents after copying template
pub async fn setup_sheet_basics(
    project: &Project,
    tree: Node,
    hub: &SheetsHub,
    spreadsheet_id: &str,
    props: google_sheets4::api::SheetProperties,
) {
    tracing::info!(
        "Copied template sheet 0 from {} into {}",
        TEMPLATE_ID,
        spreadsheet_id
    );

    rename_new_sheet(project, hub.clone(), props, spreadsheet_id).await;

    // Set B1 to "(x Folders) (y Files)" for top-level children, and C1 to a hyperlink to the top-level folder.
    let top_children: &[Node] = tree.children.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

    let mut folder_count = 0usize;
    let mut file_count = 0usize;

    for c in top_children.iter() {
        if c.file_type == InternalType::Folder {
            folder_count += 1;
        } else {
            file_count += 1;
        }
    }

    let b1_text = format!("({} Folders) ({} Files)", folder_count, file_count);

    let safe_title = project.name.replace('\'', "''");
    let range_b1 = format!("'{}'!B1", safe_title);

    let vr_b1 = google_sheets4::api::ValueRange {
        range: Some(range_b1.clone()),
        major_dimension: None,
        values: Some(vec![vec![serde_json::Value::String(b1_text)]]),
    };

    match hub
        .spreadsheets()
        .values_update(vr_b1, spreadsheet_id, &range_b1)
        .value_input_option("USER_ENTERED")
        .doit()
        .await
    {
        Ok((_resp, _)) => {
            tracing::info!("Set {} -> {}", range_b1, "B1");
        }
        Err(e) => {
            tracing::error!("Failed to set B1: {}", e);
        }
    }

    let esc_name = project.name.replace('"', "\\\"");
    let esc_url = project.source.get_top_folder_url().replace('"', "\\\"");
    let formula = format!("=HYPERLINK(\"{}\",\"{}\")", esc_url, esc_name);
    let range_c1 = format!("'{}'!C1", safe_title);
    let vr_c1 = google_sheets4::api::ValueRange {
        range: Some(range_c1.clone()),
        major_dimension: None,
        values: Some(vec![vec![serde_json::Value::String(formula)]]),
    };

    match hub
        .spreadsheets()
        .values_update(vr_c1, spreadsheet_id, &range_c1)
        .value_input_option("USER_ENTERED")
        .doit()
        .await
    {
        Ok((_resp, _)) => {
            tracing::info!("Set {} -> {}", range_c1, "C1 (hyperlink)");
        }
        Err(e) => {
            tracing::error!("Failed to set C1: {}", e);
        }
    }
}

// Write folder labels and hyperlinks
pub async fn create_folder_names(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    flat: &Vec<Node>,
    spreadsheet_id: &str
) {
    let safe_title = project.name.replace('\'', "''");
    let mut batch_values: Vec<Vec<serde_json::Value>> = Vec::new();
    let mut batch_start_row = 2;

    for (i, node) in flat.into_iter().enumerate().skip(1) {
        let (col_b, col_c) = match node.file_type {
            InternalType::Folder => {
                let esc_name = node.name.replace('"', "\\\"");
                let esc_url = node.link.replace('"', "\\\"");
                let formula = format!("=HYPERLINK(\"{}\",\"{}\")", esc_url, esc_name);
                let folder_info =
                    format!("({} Folders) ({} Files)", node.child_counts.map(|c|c.folder_count).unwrap_or(0), node.child_counts.map(|c|c.file_count).unwrap_or(0));
                (
                    serde_json::Value::String(folder_info),
                    serde_json::Value::String(formula),
                )
            }
            InternalType::File | InternalType::Link => {
                if node.idx == 0 {
                    (
                        serde_json::Value::Null,
                        serde_json::Value::String("Loose files:".to_string()),
                    )
                } else {
                    (serde_json::Value::Null, serde_json::Value::Null)
                }
            }
        };

        batch_values.push(vec![col_b, col_c]);

        // Flush batch when we reach batch size or at the end
        if batch_values.len() >= BATCH_SIZE || i == flat.len() - 1 {
            let batch_end_row = batch_start_row + batch_values.len() - 1;
            let range = format!("'{}'!B{}:C{}", safe_title, batch_start_row, batch_end_row);
            let vr = google_sheets4::api::ValueRange {
                range: Some(range.clone()),
                major_dimension: None,
                values: Some(batch_values.clone()),
            };

            match hub
                .spreadsheets()
                .values_update(vr, spreadsheet_id, &range)
                .value_input_option("USER_ENTERED")
                .doit()
                .await
            {
                Ok((_r, _)) => {
                    tracing::info!(
                        "Wrote folder links and markers to {}:{}",
                        batch_start_row,
                        batch_end_row
                    );
                }
                Err(e) => tracing::error!("Failed to write batch {}: {}", range, e),
            }

            batch_start_row += batch_values.len();
            batch_values.clear();
        }
    }
}

// Write file types to column G
pub async fn create_filetype_tags(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    req_data: &RequiredData,
    flat: &Vec<Node>,
    spreadsheet_id: &str,
) {
    match magic_db::load() {
        Ok(db) => {
            let client = reqwest::Client::new();
            let safe_title = project.name.replace('\'', "''");
            let mut batch_values: Vec<Vec<serde_json::Value>> = Vec::new();
            let mut batch_start_row = 2;

            for (i, node) in flat.into_iter().enumerate().skip(1) {
                let row = 2 + (i - 1);

                let value = match node.file_type {
                    InternalType::Folder => serde_json::Value::Null,
                    InternalType::Link => serde_json::Value::String("Web link".to_string()),
                    InternalType::File => {
                        match project.source.get_file_type(req_data, &node, &db).await {
                            Ok(value) => serde_json::Value::String(value),
                            Err(e) => {
                                warn!("Failed to detect file type for {}: {}", node.name, e);
                                serde_json::Value::String("Unknown".to_string())
                            }
                        }
                    }
                };

                batch_values.push(vec![value]);

                // Flush batch when we reach batch size or at the end
                if batch_values.len() >= BATCH_SIZE || i == flat.len() - 1 {
                    let batch_end_row = batch_start_row + batch_values.len() - 1;
                    let range = format!("'{}'!G{}:G{}", safe_title, batch_start_row, batch_end_row);
                    let vr = google_sheets4::api::ValueRange {
                        range: Some(range.clone()),
                        major_dimension: None,
                        values: Some(batch_values.clone()),
                    };

                    match hub
                        .spreadsheets()
                        .values_update(vr, spreadsheet_id, &range)
                        .value_input_option("RAW")
                        .doit()
                        .await
                    {
                        Ok((_r, _)) => {
                            tracing::info!(
                                "Wrote file types to {}:G{}",
                                batch_start_row,
                                batch_end_row
                            );
                        }
                        Err(e) => {
                            tracing::error!("Failed to write batch {}: {}", range, e);
                        }
                    }

                    batch_start_row += batch_values.len();
                    batch_values.clear();
                }
            }
        }
        Err(e) => {
            error!(
                "Failed to load filetype detection database: {}",
                e.to_string()
            );
        }
    }
}

// Write file and link names with hyperlinks
pub async fn create_file_names(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    flat: &Vec<Node>,
    spreadsheet_id: &str,
) {
    let safe_title = project.name.replace('\'', "''");
    let mut batch_values: Vec<Vec<serde_json::Value>> = Vec::new();
    let mut batch_start_row = 2;

    for (i, node) in flat.into_iter().enumerate().skip(1) {
        let row = 2 + (i - 1);

        let formula = match node.file_type {
            InternalType::File | InternalType::Link => {
                let esc_name = node.name.replace('"', "\\\"");
                let esc_url = node.link.replace('"', "\\\"");
                serde_json::Value::String(format!("=HYPERLINK(\"{}\",\"{}\")", esc_url, esc_name))
            }
            InternalType::Folder => serde_json::Value::Null,
        };

        batch_values.push(vec![formula]);

        // Flush batch when we reach batch size or at the end
        if batch_values.len() >= BATCH_SIZE || i == flat.len() - 1 {
            let batch_end_row = batch_start_row + batch_values.len() - 1;
            let range = format!("'{}'!D{}:D{}", safe_title, batch_start_row, batch_end_row);
            let vr = google_sheets4::api::ValueRange {
                range: Some(range.clone()),
                major_dimension: None,
                values: Some(batch_values.clone()),
            };

            match hub
                .spreadsheets()
                .values_update(vr, spreadsheet_id, &range)
                .value_input_option("USER_ENTERED")
                .doit()
                .await
            {
                Ok((_r, _)) => {
                    tracing::info!("Wrote file links to {}:{}", batch_start_row, batch_end_row);
                }
                Err(e) => tracing::error!("Failed to write batch {}: {}", range, e),
            }

            batch_start_row += batch_values.len();
            batch_values.clear();
        }
    }
}

// Attempt to rename the newly copied sheet to the project name
async fn rename_new_sheet(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    props: google_sheets4::api::SheetProperties,
    spreadsheet_id: &str,
) {
    let rename_req = BatchUpdateSpreadsheetRequest {
        requests: Some(vec![google_sheets4::api::Request {
            update_sheet_properties: Some(google_sheets4::api::UpdateSheetPropertiesRequest {
                properties: Some(google_sheets4::api::SheetProperties {
                    sheet_id: props.sheet_id,
                    title: Some(project.name.clone()),
                    ..Default::default()
                }),
                fields: Some(FieldMask::new(&["title"])),
            }),
            ..Default::default()
        }]),
        ..Default::default()
    };
    match hub
        .spreadsheets()
        .batch_update(rename_req, spreadsheet_id)
        .doit()
        .await
    {
        Ok((_resp, _)) => {
            tracing::info!(
                "Renamed copied sheet to \"{}\" in {}",
                project.name,
                spreadsheet_id
            );
        }
        Err(e) => {
            tracing::error!("Failed to rename copied sheet: {}", e);
        }
    }
}
