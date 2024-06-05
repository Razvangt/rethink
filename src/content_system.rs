use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};

use std::sync::Arc;

pub struct State {
    pub content: text_editor::Content,
}

impl State {
    pub fn clear(&mut self) {
        self.content = text_editor::Content::new();
    }
    pub fn change_content(&mut self, contents: Arc<String>) {
        self.content = text_editor::Content::with_text(&contents);
    }
    pub fn get_text(&self) -> String {
        self.content.text()
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.content.cursor_position()
    }
    pub fn get_mut(&mut self) -> &mut Self {
        &mut *self
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }
}
