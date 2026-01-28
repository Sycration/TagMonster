use crate::{
    CONFIG_DIR, Message, State,
    box_login::{self},
    gapi_login,
    persist::persist,
    subwindows::Subwindow,
    update,
};
use anyhow::Error;
use r#box::models::AccessToken;
use google_sheets4::{
    Sheets, hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector,
};
use iced::{
    Element,
    Length::Fill,
    Padding, Task,
    futures::FutureExt,
    widget::{Space, TextInput, button, column, row, space},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProgramSettingsState {
    pub box_key: String,
    pub box_secret: String,
    pub gapi_key: String,
    pub gapi_secret: String,
}

#[derive(Clone)]
pub enum ProgramSettingsMessage {
    UpdateBoxKey(String),
    UpdateBoxSecret(String),
    UpdateGapiKey(String),
    UpdateGapiSecret(String),
    LoginBoxButton,
    LoginBox(Result<AccessToken, String>),
    LoginGoogleButton,
    LoginGoogle(Result<Sheets<HttpsConnector<HttpConnector>>, String>),
}

fn save(ps: ProgramSettingsState) -> Task<Message> {
    Task::perform(
        async move {
            match persist(&ps, &CONFIG_DIR, "settings").await {
                Ok(_) => Message::None,
                Err(e) => {
                    tracing::error!("Error saving settings: {}", e);
                    Message::None
                }
            }
        },
        |x| x,
    )
}

pub fn handle_prog_settings(state: &mut State, event: ProgramSettingsMessage) -> Task<Message> {
    match event {
        ProgramSettingsMessage::UpdateBoxKey(k) => {
            state.program_set_state.box_key = k;
            save(state.program_set_state.clone())
        }
        ProgramSettingsMessage::UpdateBoxSecret(s) => {
            state.program_set_state.box_secret = s;
            save(state.program_set_state.clone())
        }
        ProgramSettingsMessage::UpdateGapiKey(k) => {
            state.program_set_state.gapi_key = k;
            save(state.program_set_state.clone())
        }
        ProgramSettingsMessage::UpdateGapiSecret(s) => {
            state.program_set_state.gapi_secret = s;
            save(state.program_set_state.clone())
        }
        ProgramSettingsMessage::LoginBoxButton => {
            let key = state.program_set_state.box_key.to_string();
            let secret = state.program_set_state.box_secret.to_string();
            let file = CONFIG_DIR.join("box_auth.json");
            Task::perform(box_login::get_key(key, secret, file, false), |f| {
                Message::ProgSetMessage(ProgramSettingsMessage::LoginBox(
                    f.map_err(|e| e.to_string()),
                ))
            })
        }
        ProgramSettingsMessage::LoginBox(token_response) => match token_response {
            Ok(t) => {
                state.box_token = Some(t.clone());
                state.box_config.oauth_access_token = t.access_token;
                update(state, {
                    tracing::info!("Logged in Box successfully");
                    Message::None
                })
            }
            Err(e) => {
                state.box_token = None;
                tracing::error!("Error logging in Box: {}", e);
                update(state, { Message::None })
            }
        },
        ProgramSettingsMessage::LoginGoogle(r) => match r {
            Ok(s) => {
                state.gapi_hub = Some(s);
                tracing::info!("Logged in Google successfully");
                update(state, { Message::None })
            }
            Err(e) => {
                state.gapi_hub = None;
                tracing::error!("Error logging in Google: {}", e);
                update(state, { Message::None })
            }
        },
        ProgramSettingsMessage::LoginGoogleButton => {
            let key = state.program_set_state.gapi_key.to_string();
            let secret = state.program_set_state.gapi_secret.to_string();
            let file = CONFIG_DIR.join("gapi_token.json");
            Task::perform(gapi_login::google_login(key, secret, file), |x| {
                Message::ProgSetMessage(ProgramSettingsMessage::LoginGoogle(
                    x.map_err(|e| e.to_string()),
                ))
            })
        }
    }
}

pub(crate) fn program_settings(state: &State) -> Element<Message> {
    let box_key = TextInput::new("Box.com key", &state.program_set_state.box_key)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateBoxKey(s)));
    let box_secret = TextInput::new("Box.com secret", &state.program_set_state.box_secret)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateBoxSecret(s)));

    let gapi_key = TextInput::new("Google key", &state.program_set_state.gapi_key)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateGapiKey(s)));
    let gapi_secret = TextInput::new("Google secret", &state.program_set_state.gapi_secret)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateGapiSecret(s)));

    let close = button("Close").on_press(Message::CloseWindow(Subwindow::ProgramSettings));
    let login_box = button("Login Box").on_press(Message::ProgSetMessage(
        ProgramSettingsMessage::LoginBoxButton,
    ));
    let login_google = button("Login Google").on_press(Message::ProgSetMessage(
        ProgramSettingsMessage::LoginGoogleButton,
    ));

    column![
        "Box.com information",
        box_key,
        box_secret,
        login_box,
        "Google information",
        gapi_key,
        gapi_secret,
        login_google,
        Space::new().height(Fill),
        close
    ]
    .padding(Padding::new(15.0))
    .spacing(15.0)
    .into()
}
