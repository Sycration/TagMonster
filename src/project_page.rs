use std::{env::current_dir, path::PathBuf};

use crate::{
    CONFIG_DIR, Message, Pane, State, file_tree, homepage, persist, project::Project,
    screens::Screen, subwindows::Subwindow, update,
};
use r#box::apis::folders_api::GetFoldersIdParams;
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

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    top_url: String,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    SetUrl(String),
    NewProjButton,
}

pub(crate) fn close_project(state: &mut State) -> Task<Message> {
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
        update(state, Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(project.top_folder_id))),
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
    go.chain(update(
        state,
        Message::Debug(format!("Created project \"{name}\"")),
    ))
}

pub(crate) fn open_project(state: &mut State, project: Project) -> Task<Message> {
    let name = project.name.clone();
    let id = project.top_folder_id;
    state.project = Some(project);
    state.screen = Screen::Project;
    update(state, Message::FileTreeMessage(file_tree::FileTreeMessage::InitFolder(id))).chain(
    update(state, Message::Debug(format!("Loaded project \"{name}\""))))
}

pub(crate) fn handle_new_proj_ev(state: &mut State, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::SetUrl(url) => {
            state.new_proj_state.top_url = url;
            Task::none()
        }
        NewProjEvent::NewProjButton => {
            let url = state.new_proj_state.top_url.clone();
            let id: Result<usize, _> = url.split('/').last().unwrap_or("").parse();
            let id = match id {
                Ok(id) => id,
                Err(_) => {
                    return update(state, Message::Debug("Invalid URL".to_string()));
                }
            };

            let config = state.box_config.clone();

            Task::perform(
                async move {
                    r#box::apis::folders_api::get_folders_id(
                        &config,
                        GetFoldersIdParams {
                            folder_id: id.to_string(),
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
                },
                {
                    move |x| match x {
                        Ok(f) => Message::NewProj(Project {
                            name: f.name.unwrap_or(format!("Folder {id} Project")),
                            top_folder_id: id,
                            url: url.clone(),
                        }),
                        Err(e) => Message::Debug(e.to_string()),
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
                    .map(|_| |u| Message::NewProjMessage(NewProjEvent::SetUrl(u)))
            ),
            text("Copy and paste the box folder URL here"),
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
