pub mod header;
pub mod chat;
pub mod input;

use crate::app::AssistantApp;
use crate::message::Message;
use iced::{
    widget::{column, horizontal_rule, button},
    Element, Length, theme
};

pub fn build_main_view(app: &AssistantApp) -> Element<Message> {
    column![
        header::build_header(),
        horizontal_rule(1),
        input::build_input_section(&app.input_value),
        input::build_screenshot_preview(),
        horizontal_rule(1),
        chat::build_chat_window(&app.messages),
        button("Take Screenshot")
            .on_press(Message::TakeScreenshot)
            .style(theme::Button::Secondary),
    ]
    .spacing(15)
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
