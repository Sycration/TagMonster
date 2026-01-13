use std::{env::current_dir, path::PathBuf};

use crate::{
    CONFIG_DIR, Message, Pane, State, file_tree, homepage, persist, project::Project,
    screens::Screen, subwindows::Subwindow, update,
};
use r#box::apis::folders_api::GetFoldersIdParams;
use google_sheets4::api::Spreadsheet;
use iced::{
    Alignment::Center,
    Border, Element,
    Length::{self, Fill},
    Task, Theme,
    advanced::graphics::text::cosmic_text::Font,
    border::Radius,
    widget::{
        self, Button, Column, Row, Space, TextInput, button, column, container, pane_grid, row,
        scrollable, text, text_input,
    },
};
use tracing::error;

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    top_url: String,
    sheets_url: String,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    SetBoxUrl(String),
    SetSheetsUrl(String),
    NewProjButton,
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
    let go = Task::batch([
        update(state, Message::CloseProj),
        update(state, Message::CloseWindow(Subwindow::NewProject)),
        update(
            state,
            Message::HomepageMessage(homepage::HomepageMessage::AddProject(project.clone())),
        ),
        update(
            state,
            Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(
                project.top_folder_id,
            )),
        ),
        Task::perform(
            {
                let project = project.clone();
                let dir = CONFIG_DIR.join("projects");
                async move { persist::persist(&project, &dir, project.name.as_str()).await }
            },
            |_| Message::None,
        ),
    ]);
    state.project = Some(project);
    //state.file_tree_state.path = state.new_proj_state.top_url.clone();
    state.new_proj_state = NewProjState::default();
    state.screen = Screen::Project;
    tracing::info!("Created new project \"{name}\"");
    go
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
            state.new_proj_state.top_url = url;
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

            let mut sheets_url = state.new_proj_state.sheets_url.clone();
            let spreadsheet_id = sheets_url.split('/').nth_back(1);
            let sheet_id: Result<i32, _> =
                sheets_url.split('=').last().unwrap_or_default().parse();
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

            Task::perform(
                async move {
                    tokio::try_join!(
                        async {
                            r#box::apis::folders_api::get_folders_id(
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
                            .map_err(FetchJoinError::from)
                        },
                        async {
                            hub.spreadsheets()
                                .get(&spreadsheet_id)
                                .doit()
                                .await
                                .map_err(FetchJoinError::from)
                                .and_then(|s| {
                                    let sheet_exists =
                                        s.1.sheets.as_ref().map_or(false, |sheets| {
                                            sheets.iter().any(|s| {
                                                s.properties.as_ref().map_or(false, |p| {
                                                    p.sheet_id == Some(sheet_id)
                                                })
                                            })
                                        });

                                    if sheet_exists {
                                        Ok(s)
                                    } else {
                                        Err(FetchJoinError::SheetDoesNotExist(sheet_id))
                                    }
                                })
                        }
                    )
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
                            sheet_id: sheet_id

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
    }
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
                &state.new_proj_state.top_url
            )
            .on_input_maybe(
                state
                    .box_token
                    .as_ref()
                    .map(|_| |u| Message::NewProjMessage(NewProjEvent::SetBoxUrl(u)))
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
        ]
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
