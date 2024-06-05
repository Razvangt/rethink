use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};
use iced::{Alignment, Application, Command, Element, Font, Length, Settings, Subscription};

pub fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

pub fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

pub fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e802}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}
