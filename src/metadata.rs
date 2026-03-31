use iced::{
    Alignment, Element, Length, Task,
    wgpu::wgt::error,
    widget::{
        self, Checkbox, Column, Container, Scrollable, TextInput, column, pick_list, row, rule,
        text,
    },
};
use iced_aw::wrap;
use serde::{Deserialize, Serialize};

use crate::{CONFIG_DIR, Message, State, persist};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputType {
    TextEntry {
        text: String,
    },
    SingleSelect {
        options: Vec<String>,
        which: Option<usize>,
    },
    MultiSelect {
        options: Vec<String>,
        which: Vec<usize>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputField {
    pub name: String,
    pub input_type: InputType,
}

#[derive(Debug, Clone)]
pub(crate) enum MetadataMessage {
    Text {
        field: String,
        text: String,
    },
    Single {
        field: String,
        which: usize,
    },
    Multi {
        field: String,
        which: usize,
        selected: bool,
    },
}

pub(crate) fn metadata_handle(state: &mut State, event: MetadataMessage) -> Task<Message> {
    if let Some(selected) = &state.selected {
        if let Some(project) = state.project.as_mut() {
            let file_id = selected.id.clone();
            let fields = project.entered_data.entry(file_id).or_insert_with(|| {
                project
                    .fields
                    .iter()
                    .map(|f| {
                        let t = match &f.input_type {
                            InputType::TextEntry { text } => {
                                InputType::TextEntry { text: text.clone() }
                            }
                            InputType::SingleSelect { options: _, which } => {
                                InputType::SingleSelect {
                                    options: vec![],
                                    which: *which,
                                }
                            }
                            InputType::MultiSelect { options: _, which } => {
                                InputType::MultiSelect {
                                    options: vec![],
                                    which: which.clone(),
                                }
                            }
                        };
                        InputField {
                            name: f.name.clone(),
                            input_type: t,
                        }
                    })
                    .collect()
            });
            match event {
                MetadataMessage::Text { field, text } => {
                    if let Some(input_field) = fields.iter_mut().find(|x| x.name == field) {
                        if let InputType::TextEntry { text: t } = &mut input_field.input_type {
                            *t = text;
                        }
                    }
                }
                MetadataMessage::Single { field, which } => {
                    if let Some(input_field) = fields.iter_mut().find(|x| x.name == field) {
                        if let InputType::SingleSelect { which: w, .. } =
                            &mut input_field.input_type
                        {
                            *w = Some(which);
                        }
                    }
                }
                MetadataMessage::Multi {
                    field,
                    which,
                    selected: sel,
                } => {
                    if let Some(input_field) = fields.iter_mut().find(|x| x.name == field) {
                        if let InputType::MultiSelect { which: w, .. } = &mut input_field.input_type
                        {
                            if sel {
                                if !w.contains(&which) {
                                    w.push(which);
                                }
                            } else {
                                w.retain(|&x| x != which);
                            }
                        }
                    }
                }
            }
        }
    }
    let project_clone = state.project.clone();
    Task::perform(
        async move {
            if let Some(project) = project_clone {
                let dir = CONFIG_DIR.join("projects");
                match persist::persist(&project, &dir.join(project.name.as_str()), "project").await
                {
                    Ok(_) => Message::None,
                    Err(e) => {
                        tracing::error!("Error saving project: {}", e);
                        Message::None
                    }
                }
            } else {
                tracing::error!("No project found in state when trying to save metadata");
                Message::None
            }
        },
        |x| x,
    )
}

pub fn metadata_page(state: &State) -> Element<Message> {
    if let Some(project) = &state.project {
        if let Some(selected) = &state.selected {
            let editable = state.selected.is_some();
            project
                .fields
                .iter()
                .fold(
                    Column::new(),
                    move |acc: Column<Message>, available_field: &InputField| {
                        let editable = editable.clone();
                        let input_element: Element<Message> = match &available_field.input_type {
                            InputType::TextEntry { .. } => {
                                let mut input = TextInput::new(
                                    &available_field.name,
                                    &project
                                        .entered_data
                                        .get(&selected.id)
                                        .and_then(|data| {
                                            data.iter()
                                                .find(|n| n.name == available_field.name)
                                                .map(|f| {
                                                    if let InputType::TextEntry { text } =
                                                        &f.input_type
                                                    {
                                                        text.clone()
                                                    } else {
                                                        "".to_string()
                                                    }
                                                })
                                        })
                                        .unwrap_or_else(|| "".to_string()),
                                );
                                if editable {
                                    input = input.on_input(|new_text| {
                                        Message::MetadataMessage(MetadataMessage::Text {
                                            field: available_field.name.clone(),
                                            text: new_text,
                                        })
                                    });
                                }
                                input.into()
                            }
                            InputType::SingleSelect { options, .. } => {
                                let selected_option = project
                                    .entered_data
                                    .get(&selected.id)
                                    .and_then(|data| {
                                        data.iter()
                                            .find(|n| n.name == available_field.name)
                                            .and_then(|f| {
                                                if let InputType::SingleSelect { which, .. } =
                                                    &f.input_type
                                                {
                                                    which.and_then(|w| options.get(w).cloned())
                                                } else {
                                                    None
                                                }
                                            })
                                    })
                                    .unwrap_or_else(|| "".to_string());

                                let pick = pick_list(
                                    options.as_slice(),
                                    Some(selected_option.clone()),
                                    move |selected| {
                                        if let Some(which) =
                                            options.iter().position(|o| o == &selected)
                                        {
                                            if editable {
                                                Message::MetadataMessage(MetadataMessage::Single {
                                                    field: available_field.name.clone(),
                                                    which,
                                                })
                                            } else {
                                                Message::None
                                            }
                                        } else {
                                            Message::None
                                        }
                                    },
                                );
                                pick.into()
                            }
                            InputType::MultiSelect { options, .. } => options
                                .iter()
                                .enumerate()
                                .fold(wrap::Wrap::new(), |acc, (index, option)| {
                                    let mut checkbox = Checkbox::new(
                                        project
                                            .entered_data
                                            .get(&selected.id)
                                            .and_then(|data| {
                                                data.iter()
                                                    .find(|n| n.name == available_field.name)
                                                    .and_then(|f| {
                                                        if let InputType::MultiSelect {
                                                            which,
                                                            ..
                                                        } = &f.input_type
                                                        {
                                                            Some(which.contains(&index))
                                                        } else {
                                                            None
                                                        }
                                                    })
                                            })
                                            .unwrap_or(false),
                                    )
                                    .label(option);
                                    if editable {
                                        checkbox = checkbox.on_toggle(move |t| {
                                            Message::MetadataMessage(MetadataMessage::Multi {
                                                field: available_field.name.clone(),
                                                which: index,
                                                selected: t,
                                            })
                                        });
                                    }
                                    acc.push(
                                        checkbox,
                                    )
                                })
                                .align_items(iced::Alignment::Center)
                                .spacing(16)
                                .line_spacing(5)
                                .into(),
                        };
                        acc.push(column![
                            row![
                                text(&available_field.name),
                                iced::widget::rule::horizontal(2)
                            ]
                            .align_y(Alignment::Center),
                            input_element
                        ])
                    },
                )
                .spacing(10)
                .into()
        } else {
            Container::new(text("Select a file"))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        }
    } else {
        text("ERROR NO FIELDS IN PROJECT").into()
    }
}
