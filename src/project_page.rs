use std::{
    env::current_dir,
    io::{BufReader, Cursor},
    path::PathBuf,
    usize,
};

use crate::make_sheet::create_file_names;
use crate::make_sheet::create_filetype_tags;
use crate::make_sheet::create_folder_names;
use crate::{
    CONFIG_DIR, Message, Pane, SheetsHub, State, TEMPLATE_ID, file_tree, homepage,
    make_sheet::setup_sheet_basics, persist, project::Project, screens::Screen,
    subwindows::Subwindow, update,
};
use r#box::{
    apis::{
        downloads_api::{GetFilesIdContentParams, get_files_id_content},
        files_api::GetFilesIdParams,
        folders_api::{GetFoldersIdItemsParams, GetFoldersIdParams},
        users_api::GetUsersMeParams,
        zip_downloads_api::PostZipDownloadsParams,
    },
    models::{ZipDownloadRequest, ZipDownloadRequestItemsInner, file__full},
};
use google_sheets4::{
    FieldMask,
    api::{BatchUpdateSpreadsheetRequest, Spreadsheet},
};
use iced::{
    Alignment::Center,
    Border, Element,
    Length::{self, Fill},
    Task, Theme,
    advanced::graphics::text::cosmic_text::Font,
    border::Radius,
    futures::{FutureExt, TryFutureExt, future::join_all},
    widget::{
        self, Button, Column, Row, Space, TextInput, button, column, container, pane_grid, row,
        scrollable, text, text_input,
    },
};
use pure_magic::MagicDb;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    top_url: String,
    sheets_url: String,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    SetBoxUrl(String),
    SetSheetsUrl(String),
    MakeSheet(Project, Node),
    NewProjButton,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub name: String,
    pub file_type: InternalType,
    pub id: String,
    pub idx: usize,
    pub web_link: String,
    pub children: Option<Vec<Node>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlatItem {
    pub name: String,
    pub file_type: InternalType,
    pub id: String,
    pub idx: usize,
    pub web_link: String,
    pub children: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum InternalType {
    File,
    Link,
    Folder,
}

pub(crate) fn close_project(state: &mut State) -> Task<Message> {
    if let Some(proj) = &state.project {
        tracing::info!("Closed project \"{}\"", proj.name);
    } else {
        tracing::info!("Closed project (no project open)");
    }
    state.screen = Screen::Home;
    state.project = None;
    Task::none()
}

fn fetch_children(
    configuration: r#box::apis::configuration::Configuration,
    folder_id: String,
    hostname: String,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Node>, String>> + Send>> {
    Box::pin(async move {
        let listing = r#box::apis::folders_api::get_folders_id_items(
            &configuration,
            GetFoldersIdItemsParams {
                folder_id: folder_id.clone(),
                fields: None,
                boxapi: None,
                marker: None,
                usemarker: None,
                sort: None,
                direction: None,
                offset: None,
                limit: None,
            },
        )
        .await
        .map_err(|e| format!("Box API error listing folder {}: {}", folder_id, e))?;

        let entries = listing.entries.unwrap_or_default();
        let mut nodes: Vec<Node> = Vec::with_capacity(entries.len());

        let mut folder_idx = 0;
        let mut file_idx = 0;

        info!(
            "Fetching children of folder {}: {} entries",
            folder_id,
            entries.len()
        );

        for (idx, entry) in entries.into_iter().enumerate() {
            match entry {
                r#box::models::Item::FileFull(f) => {
                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED FILE".to_string()),
                        web_link: format!("{}/file/{}", hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: file_idx,
                        file_type: InternalType::File,
                        children: None,
                    });
                    file_idx += 1;
                }
                r#box::models::Item::FolderMini(f) => {
                    let child_node =
                        fetch_children(configuration.clone(), f.id.clone(), hostname.clone())
                            .await?;
                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED FOLDER".to_string()),
                        web_link: format!("{}/folder/{}", hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: folder_idx,
                        file_type: InternalType::Folder,
                        children: Some(child_node),
                    });
                    folder_idx += 1;
                }
                r#box::models::Item::WebLink(f) => {
                    nodes.push(Node {
                        name: f.name.unwrap_or_else(|| "UNNAMED LINK".to_string()),
                        web_link: format!("{}/web_link/{}", hostname.trim_end_matches('/'), &f.id),
                        id: f.id,
                        idx: file_idx,
                        file_type: InternalType::Link,
                        children: None,
                    });
                    file_idx += 1;
                }
            };
        }

        Ok(nodes)
    })
}

async fn build_folder_tree(
    configuration: r#box::apis::configuration::Configuration,
    folder_id: String,
    hostname: String,
) -> Result<Node, String> {
    use r#box::apis::folders_api::GetFoldersIdItemsParams;

    // Build the root node for the provided folder id
    let root_listing = r#box::apis::folders_api::get_folders_id(
        &configuration,
        r#box::apis::folders_api::GetFoldersIdParams {
            folder_id: folder_id.clone(),
            fields: None,
            if_none_match: None,
            boxapi: None,
            sort: None,
            direction: None,
            offset: None,
            limit: None,
        },
    )
    .await
    .map_err(|e| format!("Box API error fetching folder {}: {}", folder_id, e))?;

    let root_name = root_listing
        .name
        .unwrap_or_else(|| format!("Folder {}", folder_id));
    let children =
        fetch_children(configuration.clone(), folder_id.clone(), hostname.clone()).await?;

    Ok(Node {
        name: root_name,
        id: folder_id,
        web_link: format!(
            "{}/folder/{}",
            hostname.trim_end_matches('/'),
            root_listing.id
        ),
        idx: 0,
        children: Some(children),
        file_type: InternalType::Folder,
    })
}

pub(crate) fn new_project(state: &mut State, project: Project) -> Task<Message> {
    let name = project.name.clone();
    if let Err(e) = std::fs::create_dir_all(CONFIG_DIR.join("projects").join(&name)) {
        warn!("Failed to create project subdirectory: {}", e);
    }
    let go = Task::batch([
        update(state, Message::CloseProj),
        update(state, Message::CloseWindow(Subwindow::NewProject)),
        update(
            state,
            Message::HomepageMessage(homepage::HomepageMessage::AddProject(project.clone())),
        ),
        Task::perform(
            {
                let project = project.clone();
                let dir = CONFIG_DIR.join("projects");
                async move { persist::persist(&project, &dir, project.name.as_str()).await }
            },
            |_| Message::None,
        )
        .chain(build_tree(state, &project))
        .chain(update(
            state,
            Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(
                project.top_folder_id,
            )),
        )),
    ]);
    state.project = Some(project);
    //state.file_tree_state.path = state.new_proj_state.top_url.clone();
    state.new_proj_state = NewProjState::default();
    state.screen = Screen::Project;
    tracing::info!("Created new project \"{name}\"");
    go
}

fn build_tree(state: &mut State, project: &Project) -> Task<Message> {
    let configuration = state.box_config.clone();
    let project_future = project.clone();
    let project_callback = project.clone();
    let client = state.box_config.client.clone();
    let token = state
        .box_token
        .as_ref()
        .and_then(|x| x.access_token.clone())
        .unwrap_or_default();
    Task::perform(
        async move {
            // Build a simple tree of nodes with name, id and web_link, recursively.

            let box_url = project_future.box_url.clone();
            let hostname = box_url.split('/').take(3).collect::<Vec<&str>>().join("/");

            match build_folder_tree(
                configuration.clone(),
                project_future.top_folder_id.to_string(),
                hostname,
            )
            .await
            {
                Ok(tree) => {
                    tracing::info!("Built project tree for {}", project_future.name,);
                    // Optionally persist the tree to disk for later use
                    let proj_name = project_future.name.clone();
                    let _ = persist::persist(
                        &tree,
                        &CONFIG_DIR.join("projects").join(&proj_name),
                        &(proj_name + "_tree"),
                    )
                    .await;
                    Ok(tree)
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to build folder tree for {}: {}",
                        project_future.name,
                        e
                    );
                    Err(e)
                }
            }
        },
        move |tree| match tree {
            Ok(t) => Message::NewProjMessage(NewProjEvent::MakeSheet(project_callback, t)),
            Err(e) => {
                tracing::error!("Error building folder tree: {}", e);
                Message::None
            }
        },
    )
}

async fn download_zip(
    configuration: r#box::apis::configuration::Configuration,
    project: Project,
    client: Client,
    token: String,
) -> Result<(File, PathBuf), String> {
    let result: Result<(File, PathBuf), String> = async {
        let path = &CONFIG_DIR
            .join("projects")
            .join(project.name.clone() + ".zip");
        let z = r#box::apis::zip_downloads_api::post_zip_downloads(
            &configuration,
            PostZipDownloadsParams {
                zip_download_request: Some(ZipDownloadRequest {
                    items: vec![ZipDownloadRequestItemsInner {
                        r#type: r#box::models::zip_download_request_items_inner::Type::Folder,
                        id: project.top_folder_id.to_string(),
                    }],
                    download_file_name: Some(project.name.clone()),
                }),
            },
        )
        .await
        .map_err(|e| format!("Failed to retrieve ZIP URL: {}", e))?;

        let url = z
            .download_url
            .ok_or_else(|| "ZIP URL had no URL in it".to_string())?;

        let resp = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Failed to open ZIP URL: {}", e))?;

        let mut file = File::create(path)
            .await
            .map_err(|e| format!("Failed to create ZIP file: {}", e))?;

        let mut content = resp.bytes_stream();

        while let Some(chunk) = content.next().await {
            let chunk = chunk.map_err(|e| format!("Failed to download ZIP chunk: {}", e))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| format!("Failed to write ZIP chunk: {}", e))?;
        }

        file.flush()
            .await
            .map_err(|e| format!("Failed to flush ZIP file: {}", e))?;

        Ok::<(File, PathBuf), String>((file, path.clone()))
    }
    .await;
    if let Err(e) = &result {
        tracing::error!("{}", e);
    } else {
        info!(
            "Downloaded folder {}",
            &CONFIG_DIR
                .join("projects")
                .join(project.name + ".zip")
                .to_string_lossy()
        )
    }
    result
}

pub(crate) fn open_project(state: &mut State, project: Project) -> Task<Message> {
    let name = project.name.clone();
    let id = project.top_folder_id;
    state.project = Some(project);
    state.screen = Screen::Project;
    tracing::info!("Opened project \"{name}\"");
    update(
        state,
        Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(id)),
    )
}

#[derive(Debug)]
pub(crate) enum FetchJoinError<T> {
    BoxApi(r#box::apis::Error<T>),
    SheetsApi(google_sheets4::Error),
    SheetDoesNotExist(i32),
}

impl<T> std::fmt::Display for FetchJoinError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchJoinError::BoxApi(e) => write!(f, "Box API error: {}", e),
            FetchJoinError::SheetsApi(e) => write!(f, "Google Sheets API error: {}", e),
            FetchJoinError::SheetDoesNotExist(s) => {
                write!(f, "Spreadsheet does not contain sheet {s}")
            }
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for FetchJoinError<T> {}

impl<T> From<r#box::apis::Error<T>> for FetchJoinError<T> {
    fn from(e: r#box::apis::Error<T>) -> Self {
        FetchJoinError::BoxApi(e)
    }
}

impl<T> From<google_sheets4::Error> for FetchJoinError<T> {
    fn from(e: google_sheets4::Error) -> Self {
        FetchJoinError::SheetsApi(e)
    }
}

pub(crate) fn handle_new_proj_ev(state: &mut State, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::SetBoxUrl(url) => {
            state.new_proj_state.top_url = url;
            Task::none()
        }
        NewProjEvent::SetSheetsUrl(url) => {
            state.new_proj_state.sheets_url = url;
            Task::none()
        }
        NewProjEvent::NewProjButton => {
            let _box_token = if let Some(t) = &state.box_token {
                t
            } else {
                error!("Not logged in to Box");
                return Task::none();
            };

            let hub = if let Some(t) = state.gapi_hub.clone() {
                t
            } else {
                error!("Google authentication is not set up");
                return Task::none();
            };

            let box_url = state.new_proj_state.top_url.clone();
            let box_id: Result<usize, _> = box_url.split('/').last().unwrap_or_default().parse();
            let box_id = match box_id {
                Ok(id) => id,
                Err(e) => {
                    return update(state, {
                        tracing::warn!("Invalid Box URL {}: {}", box_url, e);
                        Message::None
                    });
                }
            };

            let sheets_url = state.new_proj_state.sheets_url.clone();
            let spreadsheet_id = sheets_url.split('/').nth_back(1).map(|s| s.to_string());
            let sheet_id: Result<i32, _> = sheets_url.split('=').last().unwrap_or_default().parse();
            let (spreadsheet_id, sheet_id) = match (spreadsheet_id, sheet_id) {
                (Some(sp), Ok(s)) => (sp.to_string(), s),
                (None, _) => {
                    return update(state, {
                        tracing::warn!("Invalid Spreadsheet URL (no spreadsheet ID): {}", box_url);
                        Message::None
                    });
                }
                (_, Err(e)) => {
                    return update(state, {
                        tracing::warn!("Invalid Spreadsheet URL (no sheet ID): {}", box_url);
                        Message::None
                    });
                }
            };

            let config = state.box_config.clone();

            let s_id = spreadsheet_id.clone();

            Task::perform(
                async move {
                    let folder = r#box::apis::folders_api::get_folders_id(
                        &config,
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
                    .await
                    .map_err(FetchJoinError::from)?;
                    let sheet = hub
                        .spreadsheets()
                        .get(&s_id)
                        .doit()
                        .await
                        .map_err(FetchJoinError::from)
                        .and_then(|s| {
                            let sheet_exists = s.1.sheets.as_ref().map_or(false, |sheets| {
                                sheets.iter().any(|s| {
                                    s.properties
                                        .as_ref()
                                        .map_or(false, |p| p.sheet_id == Some(sheet_id))
                                })
                            });

                            if sheet_exists {
                                Ok(s)
                            } else {
                                Err(FetchJoinError::SheetDoesNotExist(sheet_id))
                            }
                        })?;
                    Ok((folder, sheet))
                },
                {
                    move |x| match x {
                        Ok((folder_res, sheet_res)) => Message::NewProj(Project {
                            name: folder_res
                                .name
                                .unwrap_or(format!("Folder {} Project", box_id)),
                            top_folder_id: box_id,
                            box_url: box_url.clone(),
                            sheets_url: sheets_url,
                            spreadsheet_id: spreadsheet_id,
                            sheet_id: sheet_id,
                        }),
                        Err(e) => match e {
                            FetchJoinError::BoxApi(error) => {
                                tracing::error!("Error fetching folder {}: {}", box_id, error);
                                Message::None
                            }
                            FetchJoinError::SheetsApi(error) => {
                                tracing::error!("Error fetching spreadsheet: {}", error);
                                Message::None
                            }
                            FetchJoinError::SheetDoesNotExist(error) => {
                                tracing::error!("Spreadsheet error: {}", error);
                                Message::None
                            }
                        },
                    }
                },
            )
        }
        NewProjEvent::MakeSheet(project, tree) => make_sheet(state, project, tree),
    }
}

fn flatten_node(node: &Node, out: &mut Vec<FlatItem>) {
    let mut child_counts = (0, 0);

    if let Some(children) = &node.children {
        for child in children {
            if child.file_type == InternalType::Folder {
                child_counts.0 += 1;
            } else {
                child_counts.1 += 1;
            }
        }
    }

    // push the folder/file itself (as a flattened entry with no children)
    out.push(FlatItem {
        name: node.name.clone(),
        file_type: node.file_type,
        id: node.id.clone(),
        idx: node.idx,
        web_link: node.web_link.clone(),
        children: child_counts,
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
            out.push(FlatItem {
                name: file.name.clone(),
                id: file.id.clone(),
                idx: file.idx,
                web_link: file.web_link.clone(),
                file_type: file.file_type,
                children: (0, 0),
            });
        }
    }
}

fn make_sheet(state: &mut State, project: Project, tree: Node) -> Task<Message> {
    let hub = if let Some(hub) = state.gapi_hub.clone() {
        hub
    } else {
        error!("Not logged in with google");
        return Task::done(Message::None);
    };
    let box_config = state.box_config.clone();
    Task::perform(
        async move {
            let mut flat: Vec<FlatItem> = Vec::new();

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
                destination_spreadsheet_id: Some(project.spreadsheet_id.clone()),
            };
            match hub
                .spreadsheets()
                .sheets_copy_to(copy_req, TEMPLATE_ID, 0)
                .doit()
                .await
            {
                Ok((_resp, props)) => {
                    setup_sheet_basics(&project, tree, &hub, props).await;

                    tokio::join!(
                        create_filetype_tags(&project, hub.clone(), box_config.clone(), &flat),
                        create_folder_names(&project, hub.clone(), &flat,),
                        create_file_names(&project, hub.clone(), &flat),
                    );
                    info!("Done making sheet");
                }
                Err(e) => {
                    tracing::error!("Failed to copy template sheet: {}", e);
                }
            }

            anyhow::Ok(())
        },
        |_| Message::None,
    )
}

pub(crate) fn new_project_view(state: &State) -> Element<Message> {
    column![
        "Create a new project",
        column![
            TextInput::new(
                "https://berkeley.app.box.com/folder/123456789",
                &state.new_proj_state.top_url
            )
            .on_input_maybe(
                state
                    .box_token
                    .as_ref()
                    .map(|_| |u| Message::NewProjMessage(NewProjEvent::SetBoxUrl(u)))
            ),
            text("Copy and paste the box folder URL here"),
            Space::new().height(10),
            TextInput::new(
                "https://docs.google.com/spreadsheets/d/123456789/edit?gid=0#gid=0",
                &state.new_proj_state.sheets_url
            )
            .on_input_maybe(
                state
                    .gapi_hub
                    .as_ref()
                    .map(|_| |u| Message::NewProjMessage(NewProjEvent::SetSheetsUrl(u)))
            ),
            text("Copy and paste the Google Sheets URL here"),
        ]
        .spacing(10),
        row![
            Space::new().width(40),
            button("Create").style(button::primary).on_press_maybe(
                state
                    .box_token
                    .as_ref()
                    .map(|_| Message::NewProjMessage(NewProjEvent::NewProjButton))
            ),
            Space::new().width(Fill),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::NewProject)),
            Space::new().width(40),
        ],
        //TODO
        text("Currently, TagMaster only creates the spreadsheet. In a future release, you will edit it through TagMaster as well.")
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}

pub(crate) fn project_page(state: &State) -> widget::Container<'_, Message> {
    container(
        pane_grid(&state.panes, |pane, current_pane, _| {
            pane_grid::Content::new(
                scrollable(
                    match current_pane {
                        Pane::FileList => container(file_tree::file_tree(&state)),
                        Pane::DataEntry => container("Data Entry"),
                        Pane::Viewer => container("Viewer"),
                    }
                    .padding(8),
                )
                .height(Length::Fill)
                .width(Length::Fill),
            )
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style {
                    border: Border {
                        color: palette.background.strong.color,
                        width: 2.0,
                        radius: Radius::new(0),
                    },
                    ..Default::default()
                }
            })
            .title_bar(
                pane_grid::TitleBar::new(widget::stack![match current_pane {
                    Pane::FileList => container(file_tree::title_bar(&state)),
                    Pane::DataEntry => container("Metadata"),
                    Pane::Viewer => container("Viewer"),
                },])
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();

                    container::Style {
                        text_color: Some(palette.background.strong.text),
                        background: Some(palette.background.strong.color.into()),
                        ..Default::default()
                    }
                })
                .padding(4),
            )
        })
        .on_resize(6, Message::PaneResized)
        .on_drag(Message::PaneSwap)
        .spacing(3),
    )
    .center_x(Length::Fill)
}
