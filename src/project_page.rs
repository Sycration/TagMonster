use std::{
    env::current_dir,
    io::{BufReader, Cursor},
    path::PathBuf,
    process::Child,
    usize,
};

// use crate::make_sheet::create_folder_names;
use crate::source::Source;
use crate::{
    CONFIG_DIR, Message, Pane, SheetsHub, State, TEMPLATE_ID, file_tree, homepage, persist,
    project::Project, screens::Screen, subwindows::Subwindow, update,
};
use crate::{box_source, local_source::LocalSource, source};
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

#[derive(Debug, Default, Clone, Copy)]
pub enum NewProjSource {
    #[default]
    Local,
    Box,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    box_url: String,
    local_path: PathBuf,
    source: NewProjSource,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    SetSource(NewProjSource),
    #[allow(unused)]
    OpenProject(Project),
    SetBoxUrl(String),
    SetLocalFolder(PathBuf),
    SelectLocalFolder,
    NewProjButton,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Node {
    pub name: String,
    pub file_type: InternalType,
    pub id: String,
    pub idx: usize,
    pub link: String,
    pub children: Option<Vec<Node>>,
    pub child_counts: Option<ChildCounts>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
pub struct ChildCounts {
    pub folder_count: usize,
    pub file_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum InternalType {
    #[default]
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

pub(crate) fn new_project(state: &mut State, project: Project) -> Task<Message> {
    let name = project.name.clone();
    if let Err(e) = std::fs::create_dir_all(CONFIG_DIR.join("projects").join(&name)) {
        warn!("Failed to create project subdirectory: {}", e);
    }
    state.project = Some(project.clone());

    let go = Task::batch([
        update(state, Message::CloseWindow(Subwindow::NewProject)),
        update(
            state,
            Message::HomepageMessage(homepage::HomepageMessage::AddProject(project.clone())),
        ),
        Task::perform(
            {
                let project = project.clone();
                let dir = CONFIG_DIR.join("projects");
                async move {
                    persist::persist(&project, &dir.join(project.name.as_str()), "project").await
                }
            },
            |_| Message::None,
        )
        .chain(update(
            state,
            Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(
                project.source.get_top_folder_id().to_string(),
            )),
        )),
    ]);
    //state.file_tree_state.path = state.new_projstate.top_url.clone();
    state.new_proj_state = NewProjState::default();
    state.screen = Screen::Project;
    tracing::info!("Created new project \"{name}\"");
    go
}

pub(crate) fn open_project(state: &mut State, project: Project) -> Task<Message> {
    let name = project.name.clone();
    let id = project.source.get_top_folder_id().to_string();
    state.project = Some(project);
    state.screen = Screen::Project;
    tracing::info!("Opened project \"{name}\"");
    update(
        state,
        Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(id)),
    )
}

pub(crate) fn handle_new_proj_ev(state: &mut State, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::SetBoxUrl(url) => {
            state.new_proj_state.box_url = url;
            Task::none()
        }
        NewProjEvent::NewProjButton => {
            let required_data = state.required_data.clone();
            let box_url = state.new_proj_state.box_url.clone();
            let local_path = state
                .new_proj_state
                .local_path
                .to_string_lossy()
                .to_string();
            let data_source = state.new_proj_state.source.clone();
            Task::perform(
                async move {
                    let source = match data_source {
                        NewProjSource::Local => source::Source::Local(
                            LocalSource::new(&local_path, &required_data)
                                .await
                                .inspect_err(|e| {
                                    error!("Failed to create local folder source: {}", e);
                                })?,
                        ),
                        NewProjSource::Box => source::Source::Box(
                            box_source::BoxSource::new(&box_url, &required_data)
                                .await
                                .inspect_err(|e| {
                                    error!("Failed to create Box source: {}", e);
                                })?,
                        ),
                    };
                    Ok(source)
                },
                |s: anyhow::Result<Source>| {
                    match s {
                        Ok(source) => {
                            let project = Project {
                                name: source.name().to_string(),
                                source,
                            };
                            Message::NewProj(project)
                        }
                        Err(_) => {
                            // error already logged
                            Message::None
                        }
                    }
                },
            )
        }
        NewProjEvent::OpenProject(project) => open_project(state, project),
        NewProjEvent::SetSource(new_proj_source) => {
            state.new_proj_state.source = new_proj_source;
            Task::none()
        }
        NewProjEvent::SetLocalFolder(path_buf) => {
            state.new_proj_state.local_path = path_buf;
            Task::none()
        }
        NewProjEvent::SelectLocalFolder => {
            let dialog = rfd::AsyncFileDialog::new();
            Task::perform(dialog.pick_folder(), |f| {
                if let Some(handle) = f {
                    info!("Selected local folder {}", handle.path().to_string_lossy());
                    Message::NewProjMessage(NewProjEvent::SetLocalFolder(
                        handle.path().to_path_buf(),
                    ))
                } else {
                    error!("Unable to select local folder");
                    Message::None
                }
            })
        }
    }
}

pub(crate) fn new_project_view(state: &State) -> Element<Message> {
    let button_row = row![
        button("Local").on_press(
            Message::NewProjMessage(NewProjEvent::SetSource(NewProjSource::Local))
        ),
        button("Box").on_press_maybe(
            state
                .box_token
                .as_ref()
                .map(|_| Message::NewProjMessage(NewProjEvent::SetSource(NewProjSource::Box)))
        ),
    ]
    .spacing(10);

    let input_section = match state.new_proj_state.source {
        NewProjSource::Local => column![
            text("Paste or select a local folder path:"),
            row![
                Button::new("Select")
                    .on_press(Message::NewProjMessage(NewProjEvent::SelectLocalFolder)),
                TextInput::<Message>::new("", &state.new_proj_state.local_path.to_string_lossy())
                    .on_input(|u| {
                        match std::fs::canonicalize(&u) {
                            Ok(p) => Message::NewProjMessage(NewProjEvent::SetLocalFolder(p)),
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
        NewProjSource::Box => column![
            text("Paste the Box folder URL:"),
            TextInput::new(
                "https://app.box.com/folder/...",
                &state.new_proj_state.box_url
            )
            .on_input(|u| Message::NewProjMessage(NewProjEvent::SetBoxUrl(u))),
            Space::new().height(10),
        ]
        .spacing(10)
        .into(),
    };

    column![
        text("Create a new project"),
        button_row,
        input_section,
        row![
            Space::new().width(Fill),
            button("Create")
                .style(button::primary)
                .on_press(Message::NewProjMessage(NewProjEvent::NewProjButton)),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::NewProject)),
            Space::new().width(40),
        ]
        .spacing(10),
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}

pub(crate) fn project_page(state: &State) -> widget::Container<'_, Message> {
    container(
        pane_grid(&state.panes, |_pane, current_pane, _| {
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
