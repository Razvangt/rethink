use iced::executor;
use iced::highlighter::{self, Highlighter};
use iced::theme::Theme;
use iced::widget::{column, horizontal_space, pick_list, row, text, text_editor};
use iced::{Alignment, Application, Command, Element, Font, Length, Settings, Subscription};
use std::ffi;
use std::path::Path;

pub mod components;
pub mod content_system;
pub mod error;
pub mod file_system;
pub mod icons;

pub fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../assets/editor-icons.ttf")
            .as_slice()
            .into()],
        default_font: Font::MONOSPACE,
        ..Settings::default()
    })
}
pub struct Editor {
    content: content_system::State,
    theme: highlighter::Theme,
    file_state: file_system::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    Files(file_system::Event),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                content: Default::default(),
                file_state: Default::default(),
            },
            file_system::open_file_command().map(Message::Files),
        )
    }

    fn title(&self) -> String {
        String::from("Editor - Iced")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ActionPerformed(action) => {
                self.file_state.is_dirty = self.file_state.is_dirty || action.is_edit();

                self.content.content.perform(action);

                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;

                Command::none()
            }
            Message::Files(event) => self
                .file_state
                .update(event, self.content.get_mut())
                .map(Message::Files),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.file_state.subscription().map(Message::Files)
    }

    fn view(&self) -> Element<Message> {
        let file_controls = self.file_state.controls_view().map(Message::Files);
        let controls = row![
            file_controls,
            horizontal_space(),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10])
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let status = row![
            text(if let Some(path) = &self.file_state.file {
                let path = path.display().to_string();

                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New file")
            }),
            horizontal_space(),
            text({
                let (line, column) = self.content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        column![
            controls,
            text_editor(&self.content.content)
                .height(Length::Fill)
                .on_action(Message::ActionPerformed)
                .highlight::<Highlighter>(
                    highlighter::Settings {
                        theme: self.theme,
                        extension: self
                            .file_state
                            .file
                            .as_deref()
                            .and_then(Path::extension)
                            .and_then(ffi::OsStr::to_str)
                            .map(str::to_string)
                            .unwrap_or(String::from("rs")),
                    },
                    |highlight, _theme| highlight.to_format()
                ),
            status,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
