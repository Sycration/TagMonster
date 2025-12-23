use std::fmt::Debug;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use r#box::apis::authorization_api::PostOauth2TokenRefreshParams;
use r#box::apis::configuration::Configuration;
use r#box::models::AccessToken;
use iced::Element;
use iced::Length;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::futures::FutureExt;
use iced::widget;
use iced::widget::pane_grid;
use iced::widget::rule;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::text_input::default;
use iced::window;
use iced::window::Id;
use iced_aw::style::colors::WHITE;
use tokio::io::AsyncReadExt;
use tokio_stream::wrappers::ReadDirStream;

use crate::file_tree::FileTreeState;
use crate::persist::retrieve;
use crate::persist::retrieve_sync;
use crate::program_settings::ProgramSettingsState;
use crate::project::Project;
use crate::screens::Screen;
use crate::subwindows::Subwindow;

mod box_login;
mod homepage;
mod persist;
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
    NewProj(Project),
    OpenProject(Project),
    HomepageMessage(homepage::HomepageMessage),
    NewProjMessage(project_page::NewProjEvent),
    FileTreeMessage(file_tree::FileTreeMessage),
    ProgSetMessage(program_settings::ProgramSettingsMessage),
    Select(PathBuf),
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
    PaneSwap(pane_grid::DragEvent),
    InitProgramSettings(ProgramSettingsState),
    InitAccessToken(Option<AccessToken>),
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
    homepage_state: homepage::HomepageState,
    file_tree_state: file_tree::FileTreeState,
    program_set_state: program_settings::ProgramSettingsState,
    box_token: Option<AccessToken>,
    box_config: Configuration,
}

static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let cd = directories::ProjectDirs::from("org", "GenEq", "TagMaster")
        .map(|pd| pd.config_local_dir().to_path_buf())
        .unwrap_or(std::env::temp_dir());
    let _ = std::fs::create_dir_all(&cd);
    let _ = std::fs::create_dir_all(&cd.join("projects"));
    cd
});

impl Default for State {
    fn default() -> Self {
        Self {
            windows: vec![],
            panes: pane_grid::State::new(Pane::FileList).0,
            project: None,
            screen: Screen::Home,
            new_proj_state: project_page::NewProjState::default(),
            statusline: String::new(),
            file_tree_state: FileTreeState::default(),
            homepage_state: homepage::HomepageState::default(),
            program_set_state: ProgramSettingsState::default(),
            selected: None,
            box_token: None,
            box_config: Configuration::default(),
        }
    }
}

pub fn main() -> iced::Result {
    iced::daemon(
        || (State::default(), Task::done(Message::Initialize)),
        update,
        view,
    )
    .theme(theme)
    .subscription(subscription)
    .run()
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
        Message::Initialize => {
            let mut flist = pane_grid::State::new(Pane::FileList);
            let viewer = flist
                .0
                .split(pane_grid::Axis::Vertical, flist.1, Pane::Viewer)
                .unwrap();
            flist
                .0
                .split(pane_grid::Axis::Horizontal, flist.1, Pane::DataEntry)
                .unwrap();

            state.panes = flist.0;

            Task::batch([
                Task::perform(
                    async {
                        let rd = tokio::fs::read_dir(CONFIG_DIR.join("projects")).await;
                        match rd {
                            Ok(mut rd) => {
                                let mut projects = vec![];
                                while let Ok(Some(f)) = rd.next_entry().await {
                                    if let Ok(mut file) = tokio::fs::File::open(f.path()).await {
                                        let mut contents = String::new();
                                        if file.read_to_string(&mut contents).await.is_ok() {
                                            if let Ok(proj) =
                                                serde_json::from_str::<Project>(&contents)
                                            {
                                                projects.push(proj);
                                            }
                                        }
                                    }
                                }
                                projects
                            }
                            Err(_e) => vec![],
                        }
                    },
                    |v| Message::HomepageMessage(homepage::HomepageMessage::InitProjects(v)),
                ),
                Task::perform(
                    {
                        async move {
                            retrieve::<ProgramSettingsState>(&CONFIG_DIR, "settings")
                                .await
                                .unwrap_or_default()
                        }
                    },
                    |res| Message::InitProgramSettings(res),
                ),
                Task::perform(
                    { async move { retrieve::<AccessToken>(&CONFIG_DIR, "auth").await.ok() } },
                    |res| Message::InitAccessToken(res),
                ),
            ])
            .chain(Task::perform(async {}, |_| Message::None))
            .chain(update(state, Message::OpenWindow(Subwindow::Main)))
        }
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
        Message::NewProj(proj) => project_page::new_project(state, proj),
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
            project_page::handle_new_proj_ev(state, new_proj_event)
        }
        Message::FileTreeMessage(file_tree_event) => {
            file_tree::file_tree_handle(&mut state.file_tree_state, file_tree_event)
        }
        Message::ProgSetMessage(prog_set_event) => {
            program_settings::handle_prog_settings(state, prog_set_event)
        }
        Message::Select(path_buf) => {
            if path_buf.exists() {
                state.selected = Some(path_buf);
            }
            Task::none()
        }
        Message::InitProgramSettings(program_settings_state) => {
            state.program_set_state = program_settings_state;
            Task::none()
        }
        Message::InitAccessToken(token_opt) => {
            if let Some(t) = &token_opt {
                state.box_config.oauth_access_token = t.access_token.clone();
            }
            state.box_token = token_opt;
            Task::none()
        }
        Message::HomepageMessage(homepage_message) => {
            homepage::handle_homepage_message(state, homepage_message)
        }
        Message::OpenProject(project) => project_page::open_project(state, project),
    }
}

fn view(state: &State, window_id: window::Id) -> Element<Message> {
    if let Some(window) = &state.windows.iter().find(|x| x.0 == window_id) {
        match window.1 {
            Subwindow::Main => main_window(state),
            Subwindow::ProjectSettings => project_settings::project_settings(state),
            Subwindow::ProgramSettings => program_settings::program_settings(state),
            Subwindow::NewProject => project_page::new_project_view(state),
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
        Screen::Home => homepage::homepage(state),
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

    widget::container(widget::column![top_bar, rule::horizontal(2), body, statusline].spacing(10))
        .padding(10)
        .into()
}
