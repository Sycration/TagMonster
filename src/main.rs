use std::fmt::Debug;
use std::process::id;

use iced::Alignment::Center;
use iced::Background;
use iced::Border;
use iced::Element;
use iced::Length;
use iced::Length::Fill;
use iced::Length::FillPortion;
use iced::Length::Shrink;
use iced::Pixels;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::advanced::graphics::futures::MaybeSend;
use iced::alignment::Horizontal;
use iced::alignment::Horizontal::Left;
use iced::application::Update;
use iced::border::Radius;
use iced::theme::palette;
use iced::widget;
use iced::widget::PaneGrid;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::button::Style;
use iced::widget::column;
use iced::widget::container;
use iced::widget::horizontal_rule;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Target;
use iced::widget::pane_grid::TitleBar;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::scrollable;
use iced::widget::span;
use iced::widget::text;
use iced::widget::text_input;
use iced::window;
use iced::window::Id;
use iced::window::Settings;
use iced_aw::Quad;
use iced_aw::style::colors::WHITE;

#[derive(Debug, Clone)]
enum Message {
    None,
    Debug(String),
    OpenLink(String),
    OpenWindow(Subwindow),
    CloseWindow(Subwindow),
    CloseWinById(Id),
    ChangeScreen(Screen),
    NewProj,
    NewProjEvent(NewProjEvent),
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
    PaneSwap(pane_grid::DragEvent),
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Screen {
    #[default]
    Home,
    Project,
}
#[derive(Debug, Clone)]
enum NewProjEvent {
    NameChange(String),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Subwindow {
    Main,
    NewProject,
    ProjectSettings,
    ProgramSettings,
}

#[derive(Debug, Clone, PartialEq)]
enum Pane {
    FileList,
    DataEntry,
    Viewer,
}

#[derive(Debug, Clone)]
struct State {
    windows: Vec<(Id, Subwindow)>,
    screen: Screen,
    panes: pane_grid::State<Pane>,
    project: Option<Project>,
    new_proj_state: NewProjState,
}

impl Default for State {
    fn default() -> Self {
        let mut flist = pane_grid::State::new(Pane::FileList);
        let viewer = flist
            .0
            .split(pane_grid::Axis::Vertical, flist.1, Pane::Viewer)
            .unwrap();
        flist
            .0
            .split(pane_grid::Axis::Horizontal, flist.1, Pane::DataEntry)
            .unwrap();
        Self {
            windows: vec![],
            panes: flist.0,
            project: None,
            screen: Screen::Home,
            new_proj_state: NewProjState::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct Project {
    name: String,
}

#[derive(Debug, Default, Clone)]
struct NewProjState {
    name: String,
}

pub fn main() -> iced::Result {
    iced::daemon("TagMaster", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(|| {
            (
                State::default(),
                Task::done(Message::OpenWindow(Subwindow::Main)), //TODO startup function/message
            )
        })
}

fn subscription(state: &State) -> Subscription<Message> {
    window::events().map(|(id, ev)| match ev {
        window::Event::Closed => Message::CloseWinById(id),

        window::Event::CloseRequested => Message::CloseWinById(id),
        _ => Message::None,
    })
}

fn theme(_state: &State, _id: Id) -> Theme {
    Theme::TokyoNight
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::None => Task::none(),
        Message::Debug(s) => {
            eprintln!("{s}");
            Task::none()
        }
        Message::ChangeScreen(screen) => {
            if state.project.is_some() && screen == Screen::Project {
                state.screen = Screen::Project;
            } else {
                state.screen = Screen::Home;
            }
            Task::none()
        }
        Message::OpenWindow(sw) => {
            let window = match sw {
                Subwindow::Main => {
                    if state.windows.iter().find(|x| x.1 == sw).is_none() {
                        let window = window::open(Settings {
                            level: window::Level::AlwaysOnBottom,

                            ..Default::default()
                        });
                        state.windows.push((window.0, sw));
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
                        window.1
                    } else {
                        Task::none()
                    }
                }
            };
            window.map(|x| Message::None)
        }
        Message::CloseWindow(sw) => {
            let old_windows = state.windows.clone();
            if let Some(id) = old_windows.iter().find(|x| x.1 == sw) {
                if id.1 == Subwindow::Main {
                    update(state, Message::CloseProj).chain(window::close(id.0).chain(iced::exit()))
                } else {
                    state.windows.retain(|w| w.1 != id.1);
                    let window = window::close(id.0);
                    window
                }
            } else {
                Task::none()
            }
        }
        Message::CloseWinById(id) => {
            if Some(Subwindow::Main) == state.windows.iter().find(|x| x.0 == id).map(|x| x.1) {
                update(state, Message::CloseProj);
                window::close(id).chain(iced::exit())
            } else {
                let window = window::close(id);
                state.windows.retain(|w| w.0 != id);
                window
            }
        }
        Message::CloseProj => {
            state.screen = Screen::Home;
            state.project = None;
            Task::none()
        }
        Message::NewProj => {
            let go = update(state, Message::CloseProj)
                .chain(update(state, Message::CloseWindow(Subwindow::NewProject)));
            state.project = Some(Project {
                name: state.new_proj_state.name.clone(),
            });
            state.new_proj_state = NewProjState::default();
            state.screen = Screen::Project;
            go
        }
        Message::PaneResized(resize_event) => {
            state.panes.resize(resize_event.split, resize_event.ratio);
            Task::none()
        }
        Message::PaneSwap(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
            Task::none()
        }
        Message::PaneSwap(_) => Task::none(),
        Message::OpenLink(s) => {
            let _ = webbrowser::open(&s);
            Task::none()
        }
        Message::NewProjEvent(new_proj_event) => {
            handle_new_proj_ev(&mut state.new_proj_state, new_proj_event)
        }
    }
}

fn handle_new_proj_ev(state: &mut NewProjState, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::NameChange(n) => {
            state.name = n;
            Task::none()
        }
    }
}

fn view(state: &State, window_id: window::Id) -> Element<Message> {
    if let Some(window) = &state.windows.iter().find(|x| x.0 == window_id) {
        match window.1 {
            Subwindow::Main => main_window(state),
            Subwindow::ProjectSettings => project_settings(state),
            Subwindow::ProgramSettings => program_settings(state),
            Subwindow::NewProject => new_project_view(&state.new_proj_state),
        }
    } else {
        text(format!(
            "ERROR\nNo Render on this Window\nID: {window_id}\nCurrent Windows: {:?}",
            state.windows
        ))
        .into()
    }
}

fn main_window(state: &State) -> Element<Message> {
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
                    widget::button("Project")
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
        Space::new(Fill, 0),
        text(
            state
                .project
                .as_ref()
                .map(|x| x.name.as_str())
                .unwrap_or("")
        ),
        Space::new(Fill, 0),
        row((0..=(if state.project.is_some() { 1 } else { 0 }))
            .rev()
            .map(|i| {
                if i == 0 {
                    widget::button("Program settings")
                        .on_press(Message::OpenWindow(Subwindow::ProgramSettings))
                        .into()
                } else {
                    widget::button("Project options")
                        .on_press(Message::OpenWindow(Subwindow::ProjectSettings))
                        .into()
                }
            }),)
        .spacing(10),
    ]
    .width(Length::Fill)
    .align_y(Center);

    let body = match state.screen {
        Screen::Home => container(
            column![
                text("TagMaster").size(80).height(FillPortion(2)),
                widget::rich_text([
                    span("A project by "),
                    span("GenEq UC Berkeley")
                        .link(Message::OpenLink(
                            "https://cejce.berkeley.edu/geneq".to_string()
                        ))
                        .underline(true)
                ])
                .height(FillPortion(1)),
                row![
                    container("New Project")
                    .align_x(Center)
                    .width(FillPortion(2)),
                    container("Recent Projects")
                        .width(FillPortion(2))
                        .align_x(Center)
                ],
                row![
                    container(
                        column![
                            button("Create Project...")
                                .on_press(Message::OpenWindow(Subwindow::NewProject)),
                        ]
                        .align_x(Center)
                        .align_x(Center)
                    )
                    .height(Fill)
                    .align_x(Center)
                    .align_y(Center)
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
                    .width(FillPortion(2)),
                    container(scrollable(
                        column![
                            //TODO project files
                        ]
                        .align_x(Center)
                        .spacing(10)
                        .padding(10)
                        .width(Fill)
                    ))
                    .width(FillPortion(2))
                    .height(Fill)
                    .align_x(Center)
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
                ]
                .height(FillPortion(5)),
                widget::rich_text([
                    span("This software is licenced under the "),
                    span("GNU General Public Licence v.3")
                        .link(Message::OpenLink(
                            "https://www.gnu.org/licenses/gpl-3.0.en.html#license-text".to_string()
                        ))
                        .underline(true),
                    span("\nSource code is available on "),
                    span("GitHub")
                        .link(Message::OpenLink(
                            "https://github.com/Sycration/TagMaster/tree/main".to_string()
                        ))
                        .underline(true)
                ])
                .height(FillPortion(1))
                .align_x(Center),
            ]
            .align_x(Center)
            .padding(30)
            .spacing(30),
        )
        .center_x(Length::Fill),
        Screen::Project => container(
            pane_grid(&state.panes, |pane, state, _| {
                pane_grid::Content::new(
                    scrollable(container("data").padding(8)).height(Length::Fill),
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
                    pane_grid::TitleBar::new(widget::stack![match state {
                        Pane::FileList => "File Tree",
                        Pane::DataEntry => "Metadata",
                        Pane::Viewer => "Viewer",
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
        .center_x(Length::Fill),
    }
    .height(Length::Fill);

    widget::container(widget::column![top_bar, horizontal_rule(2), body].spacing(10))
        .padding(10)
        .into()
}

fn new_project_view(state: &NewProjState) -> Element<Message> {
    column![
        "Create a new project",
        text_input("Project Name", &state.name)
            .on_input(|s| Message::NewProjEvent(NewProjEvent::NameChange(s))),
        row![
            button("Create")
                .style(button::primary)
                .on_press(Message::NewProj),
            Space::new(Fill, 0),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::NewProject)),
        ]
        .padding(40)
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}

fn program_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProgramSettings))
        .into()
}

fn project_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProjectSettings))
        .into()
}
