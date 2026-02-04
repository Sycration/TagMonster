use crate::{Message, State, update};
use iced::{
    Task,
    window::{self, Id, Settings, icon},
};
use image::ImageFormat;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Subwindow {
    Main,
    NewProject,
    ProjectSettings,
    ProgramSettings,
    Export,
    DeleteProj
}

pub(crate) fn open_window(state: &mut State, sw: Subwindow) -> Task<Message> {
    let window = match sw {
        Subwindow::Main => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings::default());
                state.windows.push((window.0, sw));
                tracing::debug!("Opened main window");
                window.1
            } else {
                Task::none()
            }
        }
        Subwindow::ProjectSettings => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings {
                    size: iced::Size {
                        width: 300.0,
                        height: 500.0,
                    },
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                state.windows.push((window.0, sw));
                tracing::debug!("Opened project settings window");
                window.1
            } else {
                Task::none()
            }
        }
        Subwindow::ProgramSettings => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings {
                    size: iced::Size {
                        width: 300.0,
                        height: 500.0,
                    },
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                state.windows.push((window.0, sw));
                tracing::debug!("Opened program settings window");
                window.1
            } else {
                Task::none()
            }
        }
        Subwindow::NewProject => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings {
                    size: iced::Size {
                        width: 600.0,
                        height: 400.0,
                    },
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                state.windows.push((window.0, sw));
                tracing::debug!("Opened new project window");
                window.1
            } else {
                Task::none()
            }
        }
        Subwindow::Export => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings {
                    size: iced::Size {
                        width: 600.0,
                        height: 400.0,
                    },
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                state.windows.push((window.0, sw));
                tracing::debug!("Opened export window");
                window.1
            } else {
                Task::none()
            }
        },
        Subwindow::DeleteProj => {
            if state.windows.iter().find(|x| x.1 == sw).is_none() {
                let window = window::open(Settings {
                    size: iced::Size {
                        width: 600.0,
                        height: 200.0,
                    },
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                state.windows.push((window.0, sw));
                tracing::debug!("Opened delete confirmation window");
                window.1
            } else {
                Task::none()
            }
        },
    };
    window.then(|id| {
        let icon = icon::from_file_data(include_bytes!("../icon.png"), Some(ImageFormat::Png));

        if let Ok(icon) = icon {
            window::set_icon(id, icon)
        } else {
            Task::none()
        }
    })
}

pub(crate) fn close_window(state: &mut State, sw: Subwindow) -> Task<Message> {
    let old_windows = state.windows.clone();
    if let Some(id) = old_windows.iter().find(|x| x.1 == sw) {
        if id.1 == Subwindow::Main {
            tracing::debug!("Closing main window");
            update(state, Message::CloseProj).chain(iced::exit())
        } else {
            tracing::debug!("Closing {:?} window", sw);
            state.windows.retain(|w| w.1 != id.1);
            let window = window::close(id.0);
            window
        }
    } else {
        Task::none()
    }
}

pub(crate) fn close_window_by_id(state: &mut State, id: Id) -> Task<Message> {
    if Some(Subwindow::Main) == state.windows.iter().find(|x| x.0 == id).map(|x| x.1) {
        let u = update(state, Message::CloseProj);
        tracing::debug!("Closing main window");
        u.chain(window::close(id)).chain(iced::exit())
    } else {
        let window = window::close(id);
        state.windows.retain(|w| w.0 != id);
        tracing::debug!(
            "Closing {} window",
            state
                .windows
                .iter()
                .find(|x| x.0 == id)
                .map(|x| format!("{:?}", x.1))
                .unwrap_or(format!("unknown (id {id})"))
        );
        window
    }
}
