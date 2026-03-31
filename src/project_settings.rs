use crate::{Message, State, subwindows::Subwindow};
use iced::{Element, Length, Padding, widget::{Button, Column, Space, button, column}};

pub(crate) fn project_settings(_state: &State) -> Element<Message> {
    let close: Button<_> = button("close")
        .on_press(Message::CloseWindow(Subwindow::ProjectSettings))
        .into();

    column![
        "Sort by",
        
        Space::new().height(Length::Fill),
        close
    ]
    .padding(Padding::new(15.0))
    .spacing(15.0)
    .into()


}
