use std::{
    fs,
    path::{Path, PathBuf},
};

use r#box::{
    apis::{configuration, folders_api::GetFoldersIdItemsParams},
    models::{FolderFull, FolderMini, Item, Items},
};
use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    Task,
    advanced::{Widget, widget::Text},
    widget::{Button, Column, Row, Space, button, column, row, text},
};
use tracing::info;

use crate::{Message, State, update};

#[derive(Debug, Clone)]
pub(crate) struct FileTreeState {
    pub current_folder: FolderFull, //FolderFull,
    pub parents: Vec<usize>,        //Vec<usize>
    pub contents: Vec<Item>,
}

impl Default for FileTreeState {
    fn default() -> Self {
        Self {
            current_folder: FolderFull::default(), //FolderFull::default(),
            parents: vec![],
            contents: vec![],
        }
    }
}

#[derive(Debug, Clone)]

pub(crate) enum FileTreeMessage {
    OpenFolder(usize),
    FolderReceived(FolderFull),
    UpFolder,
    InitFolder(usize),
    Update,
    UpdateReceived(Items),
}

pub(crate) fn file_tree(state: &State) -> Element<Message> {
    state
        .file_tree_state
        .contents
        .iter()
        .fold(Column::new().spacing(8), |acc, x| {
            let b = match x {
                file @ Item::FileFull(file_full) => Button::new(Text::new(
                    file_full
                        .name
                        .clone()
                        .unwrap_or(format!("!!UNNAMED FILE - ID {}", file_full.id)),
                ))
                .width(Length::Fill)
                .on_press(Message::Select(file.clone())),
                folder @ Item::FolderMini(folder_mini) => Button::new(Text::new(
                    folder_mini
                        .name
                        .clone()
                        .unwrap_or(format!("!!UNNAMED FOLDER - ID {}", folder_mini.id)),
                ))
                .width(Length::Fill)
                .on_press(Message::Select(folder.clone())),
                link @ Item::WebLink(web_link) => Button::new(Text::new(
                    web_link
                        .name
                        .clone()
                        .unwrap_or(format!("!!UNNAMED WEB LINK - ID {}", web_link.id)),
                ))
                .width(Length::Fill)
                .on_press(Message::Select(link.clone())),
            };

            acc.push(
                Row::new()
                    .push(if let Item::FolderMini(f) = x {
                        if let Ok(id) = f.id.parse() {
                            Some(Button::new("→").on_press(Message::FileTreeMessage(
                                FileTreeMessage::OpenFolder(id),
                            )))
                        } else {
                            None
                        }
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
    let name = match current_folder.name.as_deref() {
        Some(n) => n.to_string(),
        None => current_folder.id.clone(),
    };
    row![
        button("↑").height(Length::Fill).on_press_maybe(
            if state.file_tree_state.parents.is_empty() {
                None
            } else {
                Some(Message::FileTreeMessage(FileTreeMessage::UpFolder))
            }
        ),
        Space::new().width(10),
        Text::new(name)
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
    match event {
        FileTreeMessage::InitFolder(id) => {
            state.file_tree_state.parents.clear();
            update(
                state,
                Message::FileTreeMessage(FileTreeMessage::OpenFolder(id)),
            )
        }
        FileTreeMessage::OpenFolder(id) => open_folder_web(state, id),
        FileTreeMessage::FolderReceived(folder) => folder_received_web(state, folder),
        FileTreeMessage::Update => update_web(state),
        FileTreeMessage::UpFolder => {
            if let Some(parent_id) = state.file_tree_state.parents.pop() {
                update(
                    state,
                    Message::FileTreeMessage(FileTreeMessage::OpenFolder(parent_id)),
                )
            } else {
                Task::none()
            }
        }
        FileTreeMessage::UpdateReceived(items) => {
            state.file_tree_state.contents = items.entries.unwrap_or(vec![]);
            Task::none()
        }
    }
}

fn folder_received_web(state: &mut State, folder: FolderFull) -> Task<Message> {
    if let Some(project) = &state.project
        && let Ok(new_folder_id) = folder.id.parse::<usize>()
    {
        if project.top_folder_id == new_folder_id {
            state.file_tree_state.parents.clear();
        } else if state
            .file_tree_state
            .contents
            .iter()
            .filter_map(|i| {
                if let Item::FolderMini(f) = i {
                    Some(f)
                } else {
                    None
                }
            })
            .any(|i| i.id == new_folder_id.to_string())
        {
            state
                .file_tree_state
                .parents
                .push(state.file_tree_state.current_folder.id.parse().unwrap_or(0));
        } else {
            return Task::none();
        }
        state.file_tree_state.current_folder = folder;
        update(state, Message::FileTreeMessage(FileTreeMessage::Update))
    } else {
        Task::none()
    }
}

fn update_web(state: &mut State) -> Task<Message> {
    let configuration = state.box_config.clone();
    let current_folder = state.file_tree_state.current_folder.clone();
    Task::perform(
        async move {
            r#box::apis::folders_api::get_folders_id_items(
                &configuration,
                GetFoldersIdItemsParams {
                    folder_id: current_folder.id,
                    fields: None,
                    usemarker: None,
                    marker: None,
                    offset: None,
                    limit: Some(5),
                    boxapi: None,
                    sort: None,
                    direction: None,
                },
            )
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

fn open_folder_web(state: &mut State, id: usize) -> Task<Message> {
    let configuration = state.box_config.clone();
    Task::perform(
        async move {
            r#box::apis::folders_api::get_folders_id(
                &configuration,
                r#box::apis::folders_api::GetFoldersIdParams {
                    folder_id: id.to_string(),
                    fields: Some(vec!["id".to_string(), "name".to_string()]),
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
        move |f| match f {
            Ok(folder) => Message::FileTreeMessage(FileTreeMessage::FolderReceived(folder)),
            Err(e) => {
                tracing::error!("Error fetching folder {}: {}", id, e);
                Message::None
            }
        },
    )
}
