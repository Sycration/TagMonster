use crate::{
    CONFIG_DIR, Message, State, box_login::{self}, persist::persist, subwindows::Subwindow, update
};
use anyhow::Error;
use r#box::models::AccessToken;
use iced::{
    Element, Length::Fill, Padding, Task, futures::FutureExt, widget::{Space, TextInput, button, column, row, space}
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProgramSettingsState {
    box_key: String,
    box_secret: String,
}

#[derive(Debug, Clone)]
pub enum ProgramSettingsMessage {
    UpdateKey(String),
    UpdateSecret(String),
    LoginButton,
    Login(Result<AccessToken, String>),
}

pub fn handle_prog_settings(state: &mut State, event: ProgramSettingsMessage) -> Task<Message> {
    match event {
        ProgramSettingsMessage::UpdateKey(k) => {
            state.program_set_state.box_key = k;
            let ps = state.program_set_state.clone();
            Task::perform(async move {
                match persist(&ps, &CONFIG_DIR, "settings").await {
                    Ok(_) => Message::None,
                    Err(e) => Message::Debug(e.to_string())
                }
            }, |x| x)
        }
        ProgramSettingsMessage::UpdateSecret(s) => {
            state.program_set_state.box_secret = s;
                        let ps = state.program_set_state.clone();
            Task::perform(async move {
                match persist(&ps, &CONFIG_DIR, "settings").await {
                    Ok(_) => Message::None,
                    Err(e) => Message::Debug(e.to_string())
                }
            }, |x| x)
        }
        ProgramSettingsMessage::LoginButton => {
            let key = state.program_set_state.box_key.to_string();
            let secret = state.program_set_state.box_secret.to_string();
            let file = CONFIG_DIR.join("auth.json");
            Task::perform(box_login::get_key(key, secret, file, false), |f| {
                Message::ProgSetMessage(ProgramSettingsMessage::Login(f.map_err(|e| e.to_string())))
            })
        }
        ProgramSettingsMessage::Login(token_response) => match token_response {
            Ok(t) => {
                state.box_token = Some(t);
                update(state, Message::Debug(format!("Logged in successfully")))
            }
            Err(e) => update(state, Message::Debug(e.to_string())),
        },
    }
}

pub(crate) fn program_settings(state: &State) -> Element<Message> {
    let key = TextInput::new("Box.com key", &state.program_set_state.box_key)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateKey(s)));
    let secret = TextInput::new("Box.com secret", &state.program_set_state.box_secret)
        .on_input(|s| Message::ProgSetMessage(ProgramSettingsMessage::UpdateSecret(s)));

    let close = button("Close").on_press(Message::CloseWindow(Subwindow::ProgramSettings));
    let login =
        button("Login").on_press(Message::ProgSetMessage(ProgramSettingsMessage::LoginButton));

    column![key, secret, login, Space::new().height(Fill), close]
        .padding(Padding::new(15.0))
        .spacing(15.0)
        .into()
}
