use crate::{Message, State, screens::Screen, subwindows::Subwindow};
use iced::{
    Alignment::Center,
    Background,
    Length::{self, Fill},
    Theme,
    widget::{self, Space, button, row},
};

pub(crate) fn top_bar(state: &State) -> widget::Row<'_, Message> {
    let top_bar = row![
        row(
            (0..=(if state.project.is_some() { 1 } else { 0 })).map(|i| {
                if i == 0 {
                    widget::button("Home")
                        .style(|theme: &Theme, _| {
                            let palette = theme.extended_palette();

                            button::Style {
                                background: Some(Background::Color(
                                    if state.screen == Screen::Home {
                                        palette.primary.strong.color
                                    } else {
                                        palette.secondary.strong.color
                                    },
                                )),
                                ..Default::default()
                            }
                        })
                        .on_press(Message::ChangeScreen(Screen::Home))
                        .into()
                } else {
                    widget::button(row![
                        "Project: ",
                        state.project.as_ref().map_or("", |p| &p.name)
                    ])
                    .style(|theme: &Theme, _| {
                        let palette = theme.extended_palette();

                        button::Style {
                            background: Some(Background::Color(
                                if state.screen == Screen::Project {
                                    palette.primary.strong.color
                                } else {
                                    palette.secondary.strong.color
                                },
                            )),
                            ..Default::default()
                        }
                    })
                    .on_press(Message::ChangeScreen(Screen::Project))
                    .into()
                }
            }),
        )
        .spacing(10),
        Space::new().width(Fill),
        row((0..=(if state.project.is_some() { 2 } else { 0 }))
            .rev()
            .map(|i| {
                if i == 0 {
                    widget::button("Program settings")
                        .on_press(Message::OpenWindow(Subwindow::ProgramSettings))
                        .into()
                } else if i == 1 {
                    widget::button("Project options")
                        .on_press(Message::OpenWindow(Subwindow::ProjectSettings))
                        .into()
                } else {
                    widget::button("Export data")
                        .on_press(Message::OpenWindow(Subwindow::Export))
                        .into()
                }
            }),)
        .spacing(10),
    ]
    .width(Length::Fill)
    .align_y(Center);
    top_bar
}
