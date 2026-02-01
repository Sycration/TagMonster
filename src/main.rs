use std::hash::Hash;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use rustls::crypto::CryptoProvider;
use r#box::apis::authorization_api::PostOauth2TokenRefreshParams;
use r#box::apis::configuration::Configuration;
use r#box::models::AccessToken;
use r#box::models::Item;
use derive_more::Debug;
use derive_more::derive;
use google_sheets4::Sheets;
use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::hyper_util::client::legacy::connect::HttpConnector;
use iced::Color;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::advanced::Widget;
use iced::advanced::subscription;
use iced::advanced::text::highlighter::Format;
use iced::debug::Span;
use iced::futures::FutureExt;
use iced::futures::StreamExt;
use iced::widget;
use iced::widget::button;
use iced::widget::pane_grid;
use iced::widget::rich_text;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::span;
use iced::widget::text;
use iced::widget::text::Highlighter;
use iced::widget::text_editor;
use iced::widget::text_editor::Action;
use iced::widget::text_editor::Content;
use iced::widget::text_input;
use iced::widget::text_input::default;
use iced::window;
use iced::window::Id;
use iced::window::icon;
use iced_aw::style::colors::WHITE;
use iced_futures::subscription::Hasher;
use iced_futures::subscription::Recipe;
use image::ImageFormat;
use tokio::io::AsyncReadExt;
use tokio::sync::broadcast::Receiver;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::wrappers::ReceiverStream;
use tracing::debug;
use tracing::info;
use tracing::warn;

use crate::file_tree::FileTreeState;
use crate::persist::retrieve;
use crate::persist::retrieve_sync;
use crate::program_settings::ProgramSettingsState;
use crate::project::Project;
use crate::project_page::Node;
use crate::screens::Screen;
use crate::source::RequiredData;
use crate::subwindows::Subwindow;

mod box_login;
mod gapi_login;
mod homepage;
mod log;
mod persist;
mod program_settings;
mod project_page;
mod project_settings;
mod screens;
mod subwindows;
mod top_bar;
mod source;
mod export;
mod make_sheet;

mod file_tree;
mod project;

#[derive(Debug, Clone)]
enum Message {
    None,
    Debug((String, tracing::Level)),
    StatuslineGo(Action),
    Initialize,
    ToggleLogs,
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
    ExportMessage(export::ExportEvent),
    #[debug("Can't")]
    ProgSetMessage(program_settings::ProgramSettingsMessage),
    Select(Node),
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
    PaneSwap(pane_grid::DragEvent),
    InitProgramSettings(ProgramSettingsState),
    InitBoxAccessToken(Option<AccessToken>),
    InitGoogleToken,
}

#[derive(Debug, Clone, PartialEq)]
enum Pane {
    FileList,
    DataEntry,
    Viewer,
}

#[derive(Debug)]
struct State {
    windows: Vec<(Id, Subwindow)>,
    screen: Screen,
    statusline: Content,
    show_logs: bool,
    panes: pane_grid::State<Pane>,
    selected: Option<Node>,
    project: Option<project::Project>,
    new_proj_state: project_page::NewProjState,
    homepage_state: homepage::HomepageState,
    file_tree_state: file_tree::FileTreeState,
    program_set_state: program_settings::ProgramSettingsState,
    export_state: export::ExportState,
    box_token: Option<AccessToken>,
    required_data: RequiredData,
    #[debug(skip)]
    gapi_hub: Option<Sheets<HttpsConnector<HttpConnector>>>,
    log_receiver: Receiver<(String, tracing::Level)>,
}

pub static TEMPLATE_ID: &str = "1q_tfznc0LUGesvm2Yb5EqUCdhhpimJFLUkrZZJ8XvWY";
pub const BATCH_SIZE: usize = 100; 
pub type SheetsHub = google_sheets4::Sheets<
        google_sheets4::hyper_rustls::HttpsConnector<
            google_sheets4::hyper_util::client::legacy::connect::HttpConnector,
        >,
    >;

static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let cd = directories::ProjectDirs::from("org", "GenEq", "TagMonster")
        .map(|pd| pd.config_local_dir().to_path_buf())
        .unwrap_or(std::env::temp_dir());
    let _ = std::fs::create_dir_all(&cd);
    let _ = std::fs::create_dir_all(&cd.join("projects"));
    cd
});

impl State {
    fn new(rx: Receiver<(String, tracing::Level)>) -> Self {
        Self {
            windows: vec![],
            panes: pane_grid::State::new(Pane::FileList).0,
            project: None,
            screen: Screen::Home,
            new_proj_state: project_page::NewProjState::default(),
            statusline: Content::new(),
            show_logs: false,
            file_tree_state: FileTreeState::default(),
            homepage_state: homepage::HomepageState::default(),
            program_set_state: ProgramSettingsState::default(),
            export_state: export::ExportState::default(),
            selected: None,
            box_token: None,
            required_data: RequiredData::default(),
            gapi_hub: None,
            log_receiver: rx,
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    unsafe {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    CryptoProvider::install_default(rustls::crypto::ring::default_provider()).unwrap();
    let (tx, rx) = tokio::sync::broadcast::channel::<(String, tracing::Level)>(2usize.pow(16));
    let log_guard = log::init_logging(tx)?;
    let rx = std::sync::Arc::new(std::sync::Mutex::new(Some(rx)));
    iced::daemon(
        move || {
            // This should only be called once, so it's safe to unwrap here
            let rx_opt = rx.lock().unwrap().take().expect("receiver already taken");
            (State::new(rx_opt), Task::done(Message::Initialize))
        },
        update,
        view,
    )
    .title("TagMonster")
    .theme(theme)
    .subscription(|state| {
        Subscription::batch([window_subscription(state), log_subscription(state)])
    })
    .run()?;
    drop(log_guard);
    Ok(())
}

fn window_subscription(state: &State) -> Subscription<Message> {
    window::events().map(|(id, ev)| match ev {
        window::Event::Closed => Message::CloseWinById(id),
        window::Event::CloseRequested => Message::CloseWinById(id),
        _ => Message::None,
    })
}

struct LogSubscription(tokio::sync::broadcast::Receiver<(String, tracing::Level)>);

impl iced_futures::subscription::Recipe for LogSubscription {
    type Output = Message;

    fn hash(&self, state: &mut Hasher) {
        std::any::TypeId::of::<LogSubscription>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced::futures::stream::BoxStream<'static, iced_futures::subscription::Event>,
    ) -> iced::futures::stream::BoxStream<'static, Message> {
        let rx = self.0;
        let s = tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(|msg| async move {
            match msg {
                Ok(line) => Some(Message::Debug(line)),
                Err(_) => None,
            }
        });
        Box::pin(s)
    }
}

fn log_subscription(state: &State) -> Subscription<Message> {
    use iced_futures::subscription::Recipe;
    let rx = state.log_receiver.resubscribe();
    let subscription = LogSubscription(rx);
    subscription::from_recipe(subscription)
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

            Task::perform(
                // Load projects from config dir
                async {
                    let rd = tokio::fs::read_dir(CONFIG_DIR.join("projects")).await;
                    match rd {
                        Ok(mut rd) => {
                            let mut projects = vec![];
                            while let Ok(Some(f)) = rd.next_entry().await {
                                if let Ok(mut file) = tokio::fs::File::open(f.path().join("project.json")).await {
                                    let mut contents = String::new();
                                    if file.read_to_string(&mut contents).await.is_ok() {
                                        match serde_json::from_str::<Project>(&contents) {
                                            Ok(proj) => projects.push(proj),
                                            Err(e) => warn!(
                                                "Invalid project {} in project directory: {e}",
                                                f.file_name().to_string_lossy()
                                            ),
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
            )
            .chain(Task::perform(
                {
                    async move {
                        retrieve::<ProgramSettingsState>(&CONFIG_DIR, "settings")
                            .await
                            .unwrap_or_default()
                    }
                },
                |res| Message::InitProgramSettings(res),
            ))
            .chain(Task::perform(
                async move { retrieve::<AccessToken>(&CONFIG_DIR, "box_auth").await.ok() },
                |res| Message::InitBoxAccessToken(res),
            ))
            .chain(Task::done(Message::InitGoogleToken))
            .chain(Task::perform(async {}, |_| Message::None))
            .chain(update(state, Message::OpenWindow(Subwindow::Main)))
        }
        Message::Debug(s) => {
            if !state.statusline.is_empty() {
                state
                    .statusline
                    .perform(text_editor::Action::Edit(text_editor::Edit::Insert('\n')));
            }
            for c in s.0.chars() {
                state
                    .statusline
                    .perform(text_editor::Action::Edit(text_editor::Edit::Insert(c)));
            }
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
            file_tree::file_tree_handle(state, file_tree_event)
        }
        Message::ProgSetMessage(prog_set_event) => {
            program_settings::handle_prog_settings(state, prog_set_event)
        }
        Message::Select(item) => {
            state.selected = Some(item);
            Task::none()
        }
        Message::InitProgramSettings(program_settings_state) => {
            state.program_set_state = program_settings_state;
            Task::none()
        }
        Message::InitBoxAccessToken(token_opt) => {
            init_box(state, token_opt)
        }
        Message::InitGoogleToken => {
            if !state.program_set_state.gapi_key.is_empty()
                && !state.program_set_state.gapi_secret.is_empty()
            {
                let client_id = state.program_set_state.gapi_key.clone();
                let client_secret = state.program_set_state.gapi_secret.clone();
                Task::perform(
                    gapi_login::google_login(
                        client_id,
                        client_secret,
                        CONFIG_DIR.join("gapi_token.json"),
                    ),
                    |x| {
                        Message::ProgSetMessage(
                            program_settings::ProgramSettingsMessage::LoginGoogle(
                                x.map_err(|e| e.to_string()),
                            ),
                        )
                    },
                )
            } else {
                debug!("No Google API credentials set");
                Task::none()
            }
        }
        Message::HomepageMessage(homepage_message) => {
            homepage::handle_homepage_message(state, homepage_message)
        }
        Message::OpenProject(project) => project_page::open_project(state, project),
        Message::ToggleLogs => {
            state.show_logs = !state.show_logs;
            Task::none()
        }
        Message::StatuslineGo(action) => match action {
            Action::Move(_) => Task::none(),
            Action::Edit(_) => Task::none(),
            Action::Click(_) => Task::none(),
            //Action::Drag(_) => {Task::none()},
            //Action::Scroll { lines: _ } => {Task::none()},
            a @ _ => {
                state.statusline.perform(a);
                Task::none()
            }
        },
        Message::ExportMessage(export_event) => export::handle_export_event(state, export_event),
    }
}

//TODO Figure out why the generated Box code doesn't work for refreshing tokens
fn init_box(state: &mut State, token_opt: Option<AccessToken>) -> Task<Message> {
    if let Some(token) = token_opt.as_ref() {
        if let Some(refresh_token) = &token.refresh_token {
            let refresh_token = refresh_token.clone();
            let box_key = state.program_set_state.box_key.clone();
            let box_secret = state.program_set_state.box_secret.clone();
            return Task::perform(
                async move {
                    let client = reqwest::Client::new();
                    let params = reqwest::Client::new()
                        .post("https://api.box.com/oauth2/token")
                        .form(&[
                            ("grant_type", "refresh_token"),
                            ("refresh_token", &refresh_token),
                            ("client_id", &box_key),
                            ("client_secret", &box_secret),
                        ]);

                    let resp = client
                        // This never changes, so we can unwrap
                        .execute(params.build().unwrap())
                        .await;

                    match resp {
                        Ok(r) => {
                            match r.json::<AccessToken>().await {
                                Ok(new_token) => {
                                    info!("Refreshed Box token successfully");
                                    persist::persist(
                                        &new_token,
                                        &CONFIG_DIR,
                                        "box_auth",
                                    ).await.ok();
                                    Some(new_token)
                                }
                                Err(e) => {
                                    warn!("Failed to refresh Box token: {e}");
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to refresh Box token: {e}");
                            None
                        }
                    }
                },
                |result| {
                    if let Some(t) = result {
                        Message::ProgSetMessage(program_settings::ProgramSettingsMessage::LoginBox(Ok(t)))
                    } else {
                        Message::None
                    }
                },
            );
        }
    }
    Task::none()
}

fn view(state: &State, window_id: window::Id) -> Element<Message> {
    if let Some(window) = &state.windows.iter().find(|x| x.0 == window_id) {
        match window.1 {
            Subwindow::Main => main_window(state),
            Subwindow::ProjectSettings => project_settings::project_settings(state),
            Subwindow::ProgramSettings => program_settings::program_settings(state),
            Subwindow::NewProject => project_page::new_project_view(state),
            Subwindow::Export => export::export_view(state),
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

    let statusline = statusline(state.show_logs, &state.statusline);

    widget::container(widget::column![top_bar, rule::horizontal(2), body, statusline].spacing(10))
        .padding(10)
        .into()
}

fn statusline(show_all: bool, content: &Content) -> Element<Message> {
    let elem = match show_all {
        true => row![
            text_editor(&content)
                .font(iced::font::Font::MONOSPACE)
                .highlight_with::<MyHighlighter>((), Highlight::to_format)
                .on_action(|a| Message::StatuslineGo(a))
                .height(Length::FillPortion(1)),
            button("V").on_press(Message::ToggleLogs)
        ]
        .into(),
        false => {
            let text = content.text();
            let last_line = text
                .rsplit_once('\n')
                .map(|s| s.1.to_string())
                .unwrap_or_else(|| text.clone());
            let level = find_level(&last_line);

            row![
                rich_text![
                    span::<String, Font>(last_line)
                        .font(iced::font::Font::MONOSPACE)
                        .color(match level {
                            Highlight::Level(tracing::Level::ERROR) =>
                                Color::from_rgb8(255, 85, 85),
                            Highlight::Level(tracing::Level::WARN) =>
                                Color::from_rgb8(255, 195, 85),
                            Highlight::Level(tracing::Level::INFO) => Color::from_rgb8(85, 255, 85),
                            Highlight::Level(tracing::Level::DEBUG) =>
                                Color::from_rgb8(85, 195, 255),
                            Highlight::Level(tracing::Level::TRACE) =>
                                Color::from_rgb8(195, 85, 255),
                            _ => Color::from_rgb8(100, 100, 100),
                        }),
                ]
                .width(Length::Fill),
                button("<").on_press(Message::ToggleLogs)
            ]
            .into()
        }
    };

    elem
}

enum Highlight {
    Level(tracing::Level),
    Normal,
}

impl Highlight {
    fn to_format(&self, _theme: &iced::Theme) -> Format<Font> {
        let color = match self {
            Highlight::Level(level) => Some(match *level {
                tracing::Level::ERROR => Color::from_rgb8(255, 85, 85),
                tracing::Level::WARN => Color::from_rgb8(255, 195, 85),
                tracing::Level::INFO => Color::from_rgb8(85, 255, 85),
                tracing::Level::DEBUG => Color::from_rgb8(85, 195, 255),
                tracing::Level::TRACE => Color::from_rgb8(195, 85, 255),
            }),

            Highlight::Normal => None,
        };
        Format { color, font: None }
    }
}

fn highlight_line(line: &str) -> Vec<(Range<usize>, Highlight)> {
    let mut offset = 0;
    line.split(['\n'])
        .enumerate()
        .map(|(i, l)| {
            let start = offset;
            offset += l.len() + 1;
            let out = find_level(l);
            (start..offset, out)
        })
        .collect()
}

fn find_level(l: &str) -> Highlight {
    if l.starts_with("[ERROR]") {
        Highlight::Level(tracing::Level::ERROR)
    } else if l.starts_with(" [WARN]") {
        Highlight::Level(tracing::Level::WARN)
    } else if l.starts_with(" [INFO]") {
        Highlight::Level(tracing::Level::INFO)
    } else if l.starts_with("[DEBUG]") {
        Highlight::Level(tracing::Level::DEBUG)
    } else if l.starts_with("[TRACE]") {
        Highlight::Level(tracing::Level::INFO)
    } else {
        Highlight::Normal
    }
}

struct MyHighlighter {
    curr_line: usize,
}

impl Highlighter for MyHighlighter {
    type Settings = ();
    type Highlight = Highlight;
    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)>>;
    fn new(_: &Self::Settings) -> Self {
        Self { curr_line: 0 }
    }
    fn update(&mut self, _: &Self::Settings) {
        self.change_line(0);
    }

    fn change_line(&mut self, line: usize) {
        self.curr_line = line;
    }
    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        Box::new(highlight_line(line).into_iter())
    }
    fn current_line(&self) -> usize {
        self.curr_line
    }
}
