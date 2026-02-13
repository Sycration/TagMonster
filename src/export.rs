use std::path::PathBuf;

use iced::{
    Alignment::Center,
    Border, Element,
    Length::{self, Fill},
    Task,
    futures::{FutureExt, TryFutureExt, future::join_all},
    widget::{
        self, Button, Column, Row, Space, TextInput, button, column, container, pane_grid, row,
        scrollable, text, text_input,
    },
};
use tokio::fs::File;
use tracing::error;

use crate::{
    Message, State, make_csv, make_sheet, project::Project, project_page::{InternalType, Node}, source::RequiredData, subwindows::Subwindow
};

#[derive(Debug, Default, Clone, Copy)]
pub enum ExportTarget {
    #[default]
    Local,
    GoogleSheets,
}

#[derive(Debug, Clone, Default)]
pub struct ExportState {
    pub target: ExportTarget,
    pub csv_path: PathBuf,
    pub google_sheets_url: String,
}

#[derive(Debug, Clone)]
pub enum ExportEvent {
    SetExportTarget(ExportTarget),
    SetCsvPath(PathBuf),
    SelectCsvFile,
    SetGoogleSheetsUrl(String),
    ExportButton,
}

pub async fn add_top_folder_node(
    req_data: &RequiredData,
    project: &Project,
    entries: Vec<Node>,
) -> anyhow::Result<Node> {
    let mut tree = project
        .source
        .get_info(
            req_data,
            &project.source.get_top_folder_id(),
            InternalType::Folder,
        )
        .await?;
    tree.child_counts = Some(Default::default());
    for child in entries.iter() {
        if child.file_type == InternalType::Folder {
            tree.child_counts
                .get_or_insert(Default::default())
                .folder_count += 1;
        } else {
            tree.child_counts
                .get_or_insert(Default::default())
                .file_count += 1;
        }
    }
    tree.children = Some(entries);

    Ok(tree)
}


pub(crate) fn handle_export_event(state: &mut State, event: ExportEvent) -> Task<Message> {
    match event {
        ExportEvent::SetExportTarget(target) => {
            state.export_state.target = target;
            Task::none()
        }
        ExportEvent::SetCsvPath(path_buf) => {
            state.export_state.csv_path = path_buf;
            Task::none()
        }
        ExportEvent::SetGoogleSheetsUrl(url) => {
            state.export_state.google_sheets_url = url;
            Task::none()
        }
        ExportEvent::ExportButton => {
            let target = state.export_state.target.clone();
            let req_data = state.required_data.clone();
            let gapi_hub = state.gapi_hub.clone();
            let google_sheets_url = state.export_state.google_sheets_url.clone();
            let csv_path = state.export_state.csv_path.clone();
            let project = if let Some(p) = &state.project {
                p.clone()
            } else {
                tracing::error!("No project loaded for export.");
                return Task::none();
            };
            let source = project.source.clone();
            Task::perform(
                async move {
                    tracing::info!("Exporting data to {:?}", target);
                    let tree = source
                        .list_contents(&req_data, &source.get_top_folder_id(), false)
                        .await?;
                    match target {
                        ExportTarget::Local => {
                            match tokio::fs::OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(&csv_path)
                                .await
                            {
                                Ok(f) => if let Err(e) = make_csv::make_csv(&req_data, project, tree, f, &csv_path.to_string_lossy()).await {
                                    tracing::error!("Error during CSV export: {}", e);
                                    anyhow::bail!("Error during CSV export: {}", e)
                                },
                                Err(e) => {
                                    tracing::error!("Could not open file for data export: {}", e);
                                    anyhow::bail!("Could not open file for data export: {}", e)
                                }
                            }
                        }
                        ExportTarget::GoogleSheets => {
                            let spreadsheet_id = google_sheets_url
                                .split('/')
                                .nth_back(1)
                                .map(|s| s.to_string());
                            let sheet_id: Result<i32, _> = google_sheets_url
                                .split('=')
                                .last()
                                .unwrap_or_default()
                                .parse();
                            let (spreadsheet_id, _sheet_id) = match (spreadsheet_id, sheet_id) {
                                (Some(sp), Ok(s)) => (sp.to_string(), s),
                                (None, _) => {
                                    tracing::warn!(
                                        "Invalid Spreadsheet URL (no spreadsheet ID): {}",
                                        google_sheets_url
                                    );
                                    anyhow::bail!(
                                        "Invalid Spreadsheet URL (no spreadsheet ID): {}",
                                        google_sheets_url
                                    );
                                }
                                (_, Err(_)) => {
                                    tracing::warn!(
                                        "Invalid Spreadsheet URL (no sheet ID): {}",
                                        google_sheets_url
                                    );
                                    anyhow::bail!(
                                        "Invalid Spreadsheet URL (no sheet ID): {}",
                                        google_sheets_url
                                    );
                                }
                            };
                            make_sheet::make_sheet(
                                &req_data,
                                gapi_hub,
                                project,
                                tree,
                                &spreadsheet_id,
                            )
                            .await?;
                            tracing::info!("Exported data to Google Sheets.");
                        }
                    }
                    Ok(())
                },
                |x: anyhow::Result<()>| match x {
                    Ok(_) => Message::CloseWindow(Subwindow::Export),
                    Err(e) => {
                        tracing::error!("Error during export: {}", e);
                        Message::CloseWindow(Subwindow::Export)
                    }
                },
            )
        }
        ExportEvent::SelectCsvFile => {
            let name = if let Some(proj) = &state.project {
                format!("{}.csv", proj.name)
            } else {
                "export.csv".to_string()
            };
            let picker = rfd::AsyncFileDialog::new().set_file_name(name);
            Task::perform(picker.save_file(), |f|
            {
                match f {
                    Some(f) => Message::ExportMessage(ExportEvent::SetCsvPath(f.path().to_path_buf())),
                    None => {
                        error!("Failed to select file to save to");
                        Message::None
                    }
                }
            }
        )
        },
    }
}

pub(crate) fn export_view(state: &State) -> Element<Message> {
    let button_row = row![
        button("CSV file").on_press(Message::ExportMessage(ExportEvent::SetExportTarget(
            ExportTarget::Local
        ))),
        button("Google Sheets").on_press_maybe(state.gapi_hub.as_ref().map(|_| {
            Message::ExportMessage(ExportEvent::SetExportTarget(ExportTarget::GoogleSheets))
        })),
    ]
    .spacing(10);

    let input_section = match state.export_state.target {
        ExportTarget::Local => column![
            text("Select a local file path:"),
            row![
                Button::new("Select")
                    .on_press(Message::ExportMessage(ExportEvent::SelectCsvFile)),
                TextInput::<Message>::new("", &state.export_state.csv_path.to_string_lossy())
                    .on_input(|u| {
                        match std::fs::canonicalize(&u) {
                            Ok(p) => Message::ExportMessage(ExportEvent::SetCsvPath(p)),
                            Err(e) => {
                                error!("Invalid local folder path: {}", e);
                                Message::None
                            }
                        }
                    })
            ],
            Space::new().height(10)
        ]
        .spacing(10),
        ExportTarget::GoogleSheets => column![
            text("Paste the Google Sheets URL:"),
            TextInput::new(
                "https://docs.google.com/spreadsheets/...",
                &state.export_state.google_sheets_url
            )
            .on_input(|u| Message::ExportMessage(ExportEvent::SetGoogleSheetsUrl(u))),
            Space::new().height(10),
        ]
        .spacing(10)
        .into(),
    };

    column![
        text("Export Data"),
        button_row,
        input_section,
        row![
            Space::new().width(Fill),
            button("Export")
                .style(button::primary)
                .on_press(Message::ExportMessage(ExportEvent::ExportButton)),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::Export)),
            Space::new().width(40),
        ]
        .spacing(10),
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}

pub fn flatten_node(node: &Node, out: &mut Vec<Node>) {
    // push the folder/file itself (as a flattened entry with no children)
    out.push(Node {
        name: node.name.clone(),
        file_type: node.file_type,
        id: node.id.clone(),
        idx: node.idx,
        link: node.link.clone(),
        children: None,
        child_counts: node.child_counts,
    });

    if let Some(children) = &node.children {
        // separate folders and files
        let mut folders: Vec<&Node> = Vec::new();
        let mut files: Vec<&Node> = Vec::new();
        for c in children {
            if c.file_type == InternalType::Folder {
                folders.push(c);
            } else {
                files.push(c);
            }
        }

        // recurse into folders first
        for f in folders {
            flatten_node(f, out);
        }

        // then append files
        for file in files {
            out.push(Node {
                name: file.name.clone(),
                id: file.id.clone(),
                idx: file.idx,
                link: file.link.clone(),
                file_type: file.file_type,
                children: None,
                child_counts: None,
            });
        }
    }
}
