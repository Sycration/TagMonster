use crate::{Message, State, project::Project, subwindows::Subwindow};
use iced::{
    Alignment::Center,
    Border,
    Length::{self, Fill, FillPortion},
    Task, Theme,
    border::Radius,
    widget::{self, Space, button, column, container, row, scrollable, span, text},
};

#[derive(Debug, Clone)]
pub(crate) struct HomepageState {
    projects: Vec<Project>,
}

impl Default for HomepageState {
    fn default() -> Self {
        Self { projects: vec![] }
    }
}

#[derive(Debug, Clone)]

pub(crate) enum HomepageMessage {
    AddProject(Project),
    InitProjects(Vec<Project>),
}

pub(crate) fn handle_homepage_message(state: &mut State, msg: HomepageMessage) -> Task<Message> {
    match msg {
        HomepageMessage::AddProject(proj) => {
            state.homepage_state.projects.push(proj);
        }
        HomepageMessage::InitProjects(projects) => {
            state.homepage_state.projects = projects;
        }
    }

    Task::none()
}
pub(crate) fn homepage<'a>(state: &'a State) -> widget::Container<'a, Message> {
    container(
        column![
            container(text("TagMaster").size(80)).height(80),
            Space::new().height(25),
            widget::rich_text([
                span("A project by "),
                span("GenEq UC Berkeley")
                    .link("https://cejce.berkeley.edu/geneq")
                    .underline(true)
            ])
            .on_link_click(|l: String| Message::OpenLink(l))
            .height(25),
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
                    state
                        .homepage_state
                        .projects
                        .iter()
                        .fold(column![], |col, proj| {
                            col.push(
                                button(proj.name.as_str())
                                    .on_press(Message::OpenProject(proj.clone()))
                                    .width(Fill),
                            )
                        },)
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
            ],
            Space::new().height(10),
            widget::rich_text([
                span("This software is licenced under the "),
                span("GNU General Public Licence v.3")
                    .link("https://www.gnu.org/licenses/gpl-3.0.en.html#license-text")
                    .underline(true),
                span("\nSource code is available on "),
                span("GitHub")
                    .link("https://github.com/Sycration/TagMaster/tree/main")
                    .underline(true)
            ])
            .on_link_click(|l: String| Message::OpenLink(l))
            .align_x(Center),
            Space::new()
        ]
        .align_x(Center),
    )
    .center_x(Length::Fill)
}
