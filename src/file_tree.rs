use std::{
    fs,
    path::{Path, PathBuf},
};

use r#box::{
    apis::{configuration, folders_api::GetFoldersIdItemsParams},
    models::{FolderFull, FolderMini, Item, Items},
};
use google_sheets4::yup_oauth2::error;
use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    Task,
    advanced::{Widget, widget::Text},
    wgpu::hal::auxil::db,
    widget::{Button, Column, Row, Space, button, column, row, text},
};
use tracing::error;
use tracing::info;

use crate::{
    Message, State,
    project_page::{InternalType, Node},
    update,
};

#[derive(Debug, Clone)]
pub(crate) struct FileTreeState {
    pub current_folder: Node,
    pub parents: Vec<String>,
    pub contents: Vec<Node>,
}

impl Default for FileTreeState {
    fn default() -> Self {
        Self {
            current_folder: Node::default(), //FolderFull::default(),
            parents: vec![],
            contents: vec![],
        }
    }
}

#[derive(Debug, Clone)]

pub(crate) enum FileTreeMessage {
    OpenFolder(String),
    FolderReceived(Node),
    UpFolder,
    InitFolder(String),
    Update,
    UpdateReceived(Vec<Node>),
}

pub(crate) fn file_tree(state: &State) -> Element<Message> {
    state
        .file_tree_state
        .contents
        .iter()
        .fold(Column::new().spacing(8), |acc, x| {
            let b = match x.file_type {
                InternalType::File => Button::new(Text::new(x.name.clone()))
                    .width(Length::Fill)
                    .on_press(Message::Select(x.clone())),
                InternalType::Folder => Button::new(Text::new(x.name.clone()))
                    .width(Length::Fill)
                    .on_press(Message::Select(x.clone())),
                InternalType::Link => Button::new(Text::new(x.name.clone()))
                    .width(Length::Fill)
                    .on_press(Message::Select(x.clone())),
            };

            acc.push(
                Row::new()
                    .push(if x.file_type == InternalType::Folder {
                        Some(Button::new("→").on_press(Message::FileTreeMessage(
                            FileTreeMessage::OpenFolder(x.id.clone()),
                        )))
                    } else {
                        None
                    })
                    .push(b)
                    .width(Length::Fill)
                    .spacing(5),
            )
        })
        .into()
}

pub(crate) fn title_bar(state: &State) -> Element<Message> {
    let current_folder = &state.file_tree_state.current_folder;
    row![
        button("↑").height(Length::Fill).on_press_maybe(
            if state.file_tree_state.parents.is_empty() {
                None
            } else {
                Some(Message::FileTreeMessage(FileTreeMessage::UpFolder))
            }
        ),
        Space::new().width(10),
        Text::new(&current_folder.name)
            .font(iced::font::Font::MONOSPACE)
            .height(Length::Fill)
            .align_y(Center)
            .width(Length::Shrink),
        Space::new().width(Fill),
        button("⟳")
            .height(Length::Fill)
            .on_press(Message::FileTreeMessage(FileTreeMessage::Update)),
    ]
    .height(28)
    .into()
}

pub(crate) fn file_tree_handle(state: &mut State, event: FileTreeMessage) -> Task<Message> {
    state.file_tree_state.contents.clear();

    match event {
        FileTreeMessage::InitFolder(id) => {
            state.file_tree_state.parents.clear();
            state.file_tree_state.current_folder.id = id.to_string();
            open_folder_web(state, id)
        }
        FileTreeMessage::OpenFolder(id) => {
            let current_id = state.file_tree_state.current_folder.id.clone();
            state.file_tree_state.parents.push(current_id);
            open_folder_web(state, id)
        }
        FileTreeMessage::FolderReceived(folder) => folder_received_web(state, folder),
        FileTreeMessage::Update => update_web(state),
        FileTreeMessage::UpFolder => {
            if let Some(parent_id) = state.file_tree_state.parents.pop() {
                open_folder_web(state, parent_id)
            } else {
                Task::none()
            }
        }
        FileTreeMessage::UpdateReceived(items) => {
            state.file_tree_state.contents = items;
            Task::none()
        }
    }
}

fn folder_received_web(state: &mut State, folder: Node) -> Task<Message> {
    if let Some(_project) = &state.project {
        state.file_tree_state.current_folder = folder;
        update(state, Message::FileTreeMessage(FileTreeMessage::Update))
    } else {
        error!("Invalid folder ID: {}", folder.id);
        Task::none()
    }
}

fn update_web(state: &mut State) -> Task<Message> {
    let required_data = state.required_data.clone();
    let current_folder = state.file_tree_state.current_folder.clone();
    let project = match state.project.clone() {
        Some(p) => p,
        None => {
            error!("No project loaded");
            return Task::none();
        }
    };
    Task::perform(
        async move {
            project
                .source
                .list_contents(&required_data, &current_folder.id, true)
                .await
        },
        |f| match f {
            Ok(items) => Message::FileTreeMessage(FileTreeMessage::UpdateReceived(items)),
            Err(e) => {
                tracing::error!("Error fetching folder items: {}", e);
                Message::None
            }
        },
    )
}

fn open_folder_web(state: &mut State, id: String) -> Task<Message> {
    let required_data = state.required_data.clone();
    let project = match state.project.clone() {
        Some(p) => p,
        None => {
            error!("No project loaded");
            return Task::none();
        }
    };
    let item_id = id.clone();
    Task::perform(
        async move {
            project
                .source
                .get_info(&required_data, &item_id, InternalType::Folder)
                .await
        },
        move |f| match f {
            Ok(folder) => Message::FileTreeMessage(FileTreeMessage::FolderReceived(folder)),
            Err(e) => {
                tracing::error!("Error fetching folder {}: {}", id, e);
                Message::None
            }
        },
    )
}
