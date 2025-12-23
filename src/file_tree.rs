use std::{env::current_dir, fs::read_dir, path::PathBuf};

use iced::{
    Alignment::Center, Element, Length, Task, advanced::{Widget, widget::Text}, widget::{Button, Column, Row, Space, button, column, row, text}
};

use crate::Message;

#[derive(Debug, Clone)]
pub(crate) struct FileTreeState {
    pub path: PathBuf,
}

impl Default for FileTreeState {
    fn default() -> Self {
        Self {
            path: current_dir().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]

pub(crate) enum FileTreeMessage {
    SetFolder(PathBuf)
}

pub(crate) fn file_tree(state: &FileTreeState) -> Element<Message> {
    read_dir(&state.path)
        .unwrap()
        .filter_map(|x| x.ok())
        .fold(Column::new().spacing(8), |acc, x| {
            let b =
                Button::new(Text::new(x.file_name().to_string_lossy().to_string())/*/.font(iced::font::Font::MONOSPACE)*/).width(Length::Fill)
                .on_press(Message::Select(x.path()));
            acc.push(
                Row::new()
                    .push(x.file_type().ok().and_then(|ft| {
                        if ft.is_dir() {
                            Some(Button::new("→").on_press(Message::FileTreeMessage(FileTreeMessage::SetFolder(x.path()))))
                        } else {
                            None
                        }
                    }))
                    .push(b)
                    .width(Length::Fill)
                    .spacing(5),
            )
        })
        .into()
}

pub(crate) fn title_bar(state: &FileTreeState) -> Element<Message> {
    row![
        button("↑").height(Length::Fill).on_press_maybe(state.path.parent().map(|p|Message::FileTreeMessage(FileTreeMessage::SetFolder(p.to_path_buf())))),
        Space::new().width(10),
        Text::new(state.path.to_string_lossy().to_string()).font(iced::font::Font::MONOSPACE).height(Length::Fill).align_y(Center).width(Length::Shrink),
    ].height(28).into()
}

pub(crate) fn file_tree_handle(state: &mut FileTreeState, event: FileTreeMessage) -> Task<Message> {
    match event {
        FileTreeMessage::SetFolder(path_buf) => {
            if path_buf.is_dir() {
                state.path = path_buf;
            }
        },
    }
    Task::none()
}
