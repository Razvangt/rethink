use crate::error::Error;

use crate::Editor;
use iced::keyboard;
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};
use iced::{Alignment, Application, Command, Element, Font, Length, Settings, Subscription};
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::components::action;
use crate::content_system;
use crate::icons::*;
pub struct State {
    pub file: Option<PathBuf>,
    pub is_loading: bool,
    pub is_dirty: bool,
}

pub async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

pub async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

#[derive(Debug, Clone)]
pub enum Event {
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}
pub fn open_file_command() -> Command<Event> {
    Command::perform(load_file(default_file()), Event::FileOpened)
}
impl State {
    pub fn update(
        &mut self,
        message: Event,
        content: &mut content_system::State,
    ) -> iced::Command<Event> {
        match message {
            Event::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    content.clear()
                }

                iced::Command::none()
            }
            Event::OpenFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(open_file(), Event::FileOpened)
                }
            }
            Event::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    content.change_content(contents);
                }

                Command::none()
            }
            Event::SaveFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(
                        save_file(self.file.clone(), content.get_text()),
                        Event::FileSaved,
                    )
                }
            }
            Event::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Command::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Event> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => Some(Event::SaveFile),
            _ => None,
        })
    }

    pub fn controls_view(&self) -> Element<Event> {
        row![
            action(new_icon(), "New file", Some(Event::NewFile)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Event::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(Event::SaveFile)
            ),
        ]
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            file: None,
            is_dirty: false,
            is_loading: true,
        }
    }
}
