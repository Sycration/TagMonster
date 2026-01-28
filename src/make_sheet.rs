use std::io::Cursor;

use r#box::apis::downloads_api::{GetFilesIdContentParams, get_files_id_content};
use google_sheets4::{FieldMask, api::BatchUpdateSpreadsheetRequest};
use iced::{futures::future::join_all};
use pure_magic::MagicDb;
use reqwest::Client;
use tokio_stream::StreamExt;
use tracing::{error, warn, debug};

use crate::{SheetsHub, TEMPLATE_ID, project::Project, project_page::{FlatItem, InternalType, Node}};


pub async fn setup_sheet_basics(
    project: &Project,
    tree: Node,
    hub: &SheetsHub,
    props: google_sheets4::api::SheetProperties,
) {
    tracing::info!(
        "Copied template sheet 0 from {} into {}",
        TEMPLATE_ID,
        project.spreadsheet_id
    );

    rename_new_sheet(project, hub.clone(), props).await;

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
        .values_update(vr_b1, project.spreadsheet_id.as_str(), &range_b1)
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
    let esc_url = project.box_url.replace('"', "\\\"");
    let formula = format!("=HYPERLINK(\"{}\",\"{}\")", esc_url, esc_name);
    let range_c1 = format!("'{}'!C1", safe_title);
    let vr_c1 = google_sheets4::api::ValueRange {
        range: Some(range_c1.clone()),
        major_dimension: None,
        values: Some(vec![vec![serde_json::Value::String(formula)]]),
    };

    match hub
        .spreadsheets()
        .values_update(vr_c1, &project.spreadsheet_id, &range_c1)
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

pub async fn create_folder_names(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    flat: &Vec<FlatItem>,
) {
    // Write folder hyperlinks and "Loose files:" markers in one pass.
    let futures = flat.into_iter().enumerate().skip(1).map(|(i, node)| {
        let safe_title = project.name.replace('\'', "''");
        let hub = hub.clone();
        async move {
            let row = 2 + (i - 1);

            match node.file_type {
                InternalType::Folder => {
                    let esc_name = node.name.replace('"', "\\\"");
                    let esc_url = node.web_link.replace('"', "\\\"");
                    let formula = format!("=HYPERLINK(\"{}\",\"{}\")", esc_url, esc_name);
                    let folder_info =
                        format!("({} Folders) ({} Files)", node.children.0, node.children.1);
                    let range = format!("'{}'!B{}:C{}", safe_title, row, row);
                    let vr = google_sheets4::api::ValueRange {
                        range: Some(range.clone()),
                        major_dimension: None,
                        values: Some(vec![vec![
                            serde_json::Value::String(folder_info),
                            serde_json::Value::String(formula),
                        ]]),
                    };

                    match hub
                        .spreadsheets()
                        .values_update(vr, &project.spreadsheet_id, &range)
                        .value_input_option("USER_ENTERED")
                        .doit()
                        .await
                    {
                        Ok((_r, _)) => tracing::info!("Wrote folder link to {}", range),
                        Err(e) => tracing::error!("Failed to write {}: {}", range, e),
                    }
                }
                InternalType::File | InternalType::Link => {
                    if node.idx == 0 {
                        let range = format!("'{}'!C{}", safe_title, row);
                        let vr = google_sheets4::api::ValueRange {
                            range: Some(range.clone()),
                            major_dimension: None,
                            values: Some(vec![vec![serde_json::Value::String(
                                "Loose files:".to_string(),
                            )]]),
                        };

                        match hub
                            .spreadsheets()
                            .values_update(vr, &project.spreadsheet_id, &range)
                            .value_input_option("USER_ENTERED")
                            .doit()
                            .await
                        {
                            Ok((_r, _)) => tracing::info!("Wrote marker to {}", range),
                            Err(e) => tracing::error!("Failed to write {}: {}", range, e),
                        }
                    }
                }
            }
        }
    });

    join_all(futures).await;
}

pub async fn create_filetype_tags(
    project: &Project,
    hub: google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >,
    box_config: r#box::apis::configuration::Configuration,
    flat: &Vec<FlatItem>,
) {
    match magic_db::load() {
        Ok(db) => {
            {
                // Iterate flattened entries (skip the root at index 0). Place results starting at row 2.
                let client = reqwest::Client::new();
                let futures = flat.into_iter().enumerate().skip(1).map(|(i, node)| {
                    let box_config = box_config.clone();
                    let client = client.clone();
                    let db = db.clone();
                    let safe_title = project.name.replace('\'', "''");
                    let hub = hub.clone();
                    let id = node.id.clone();
                    async move {
                        let row = 2 + (i - 1); // top-level is row 1, first child -> row 2

                        let value = match node.file_type {
                            InternalType::Folder => {
                                // skip folders
                                return;
                            }
                            InternalType::Link => "Web link".to_string(),
                            InternalType::File => {
                                match detect_file_type(node, box_config, client, db, id).await {
                                    Some(value) => value,
                                    None => return,
                                }
                            }
                        };

                        // Write the result into column G for this row
                        let range = format!("'{}'!G{}", safe_title, row);
                        let vr = google_sheets4::api::ValueRange {
                            range: Some(range.clone()),
                            major_dimension: None,
                            values: Some(vec![vec![serde_json::Value::String(value)]]),
                        };
                        match hub
                            .spreadsheets()
                            .values_update(vr, &project.spreadsheet_id, &range)
                            .value_input_option("RAW")
                            .doit()
                            .await
                        {
                            Ok((_r, _)) => {
                                tracing::info!("Wrote file type to {}", range);
                            }
                            Err(e) => {
                                tracing::error!("Failed to write {}: {}", range, e);
                            }
                        }
                    }
                });

                join_all(futures).await;
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

pub async fn detect_file_type(
    node: &FlatItem,
    box_config: r#box::apis::configuration::Configuration,
    client: Client,
    db: MagicDb,
    id: String,
) -> Option<String> {
    let resp = get_files_id_content(
        &box_config,
        GetFilesIdContentParams {
            file_id: id,
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
            return None;
        }
    };
    let token = match box_config.oauth_access_token.clone() {
        Some(t) => t,
        None => {
            error!("Not logged in to Box");
            return None;
        }
    };
    let resp = match client.get(url.to_owned()).bearer_auth(&token).send().await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to download file {}: {}", node.name, e.to_string());
            return None;
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
                error!("Failed to download file {}: {}", &node.name, e);
                return None;
            }
        };
        buf.extend_from_slice(&chunk);
    }
    debug!("downloaded file {}", &node.name);
    let mut cursor = Cursor::new(buf);
    let detected = match db.best_magic(&mut cursor) {
        Ok(result) => {
            // pick the first sensible result if present
            result.message()
        }
        Err(_) => {
            warn!("Failed to analyze file {}", &node.name);
            "Unknown".to_string()
        }
    };
    Some(detected)
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
        .batch_update(rename_req, &project.spreadsheet_id)
        .doit()
        .await
    {
        Ok((_resp, _)) => {
            tracing::info!(
                "Renamed copied sheet to \"{}\" in {}",
                project.name,
                project.spreadsheet_id
            );
        }
        Err(e) => {
            tracing::error!("Failed to rename copied sheet: {}", e);
        }
    }
}
