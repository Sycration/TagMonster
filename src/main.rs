use std::fmt::Debug;
use std::path::PathBuf;

use iced::Element;
use iced::Length;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::widget;
use iced::widget::horizontal_rule;
use iced::widget::pane_grid;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::text_input::default;
use iced::window;
use iced::window::Id;
use iced_aw::style::colors::WHITE;

use crate::file_tree::FileTreeState;
use crate::screens::Screen;
use crate::subwindows::Subwindow;


mod homepage;
mod program_settings;
mod project_page;
mod project_settings;
mod screens;
mod subwindows;
mod top_bar;

mod file_tree;
mod project;

#[derive(Debug, Clone)]
enum Message {
    None,
    Debug(String),
    Initialize,
    OpenLink(String),
    OpenWindow(Subwindow),
    CloseWindow(Subwindow),
    CloseWinById(Id),
    ChangeScreen(Screen),
    NewProj,
    NewProjMessage(project_page::NewProjEvent),
    FileTreeMessage(file_tree::FileTreeMessage),
    Select(PathBuf),
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
    PaneSwap(pane_grid::DragEvent),
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
    statusline: String,
    panes: pane_grid::State<Pane>,
    selected: Option<PathBuf>,
    project: Option<project::Project>,
    new_proj_state: project_page::NewProjState,
    file_tree_state: file_tree::FileTreeState,
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
            new_proj_state: project_page::NewProjState::default(),
            statusline: String::new(),
            file_tree_state: FileTreeState::default(),
            selected: None,
        }
    }
}

pub fn main() -> iced::Result {
    iced::daemon("TagMaster", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(|| {
            (
                State::default(),
                Task::done(Message::Initialize), //TODO startup function/message
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

pub(crate) fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::None => Task::none(),
        Message::Initialize => update(state, Message::Debug("Initialized".to_string()))
            .chain(update(state, Message::OpenWindow(Subwindow::Main))),
        Message::Debug(s) => {
            eprintln!("{}", &s);
            state.statusline = s;
            Task::none()
        }
        Message::ChangeScreen(screen) => screens::change_screen(state, screen),
        Message::OpenWindow(sw) => subwindows::open_window(state, sw),
        Message::CloseWindow(sw) => subwindows::close_window(state, sw),
        Message::CloseWinById(id) => subwindows::close_window_by_id(state, id),
        Message::CloseProj => project_page::close_project(state),
        Message::NewProj => project_page::new_project(state),
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
        Message::NewProjMessage(new_proj_event) => {
            project_page::handle_new_proj_ev(&mut state.new_proj_state, new_proj_event)
        }
        Message::FileTreeMessage(file_tree_event) => {
            file_tree::file_tree_handle(&mut state.file_tree_state, file_tree_event)
        }
        Message::Select(path_buf) => {
            if path_buf.exists() {
                state.selected = Some(path_buf);
            }
            Task::none()
        },
    }
}

fn view(state: &State, window_id: window::Id) -> Element<Message> {
    if let Some(window) = &state.windows.iter().find(|x| x.0 == window_id) {
        match window.1 {
            Subwindow::Main => main_window(state),
            Subwindow::ProjectSettings => project_settings::project_settings(state),
            Subwindow::ProgramSettings => program_settings::program_settings(state),
            Subwindow::NewProject => project_page::new_project_view(&state.new_proj_state),
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
    let top_bar = top_bar::top_bar(state);

    let body = match state.screen {
        Screen::Home => homepage::homepage(),
        Screen::Project => project_page::project_page(state),
    }
    .height(Length::Fill);

    let statusline = text_input("", &state.statusline)
        .font(iced::font::Font::MONOSPACE)
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            text_input::Style {
                value: WHITE,
                ..default(theme, status)
            }
        });

    widget::container(widget::column![top_bar, horizontal_rule(2), body, statusline].spacing(10))
        .padding(10)
        .into()
}
