use crate::message::Message;
use iced::{
    theme,
    widget::{column, scrollable, text},
    Element, Length,
};

pub fn build_chat_window(messages: &[(String, String)]) -> Element<Message> {
    let chat_column = column(
        messages
            .iter()
            .map(|(sender, msg)| {
                text(format!("[{}]: {}", sender, msg))
                    .size(15)
                    .style(theme::Text::Color([0.95, 0.95, 0.95].into()))
                    .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(6);

    scrollable(chat_column)
        .height(Length::Fill)
        .into()
}

pub fn build_message_bubble(sender: &str, message: &str) -> Element<'static, Message> {
    let message_style = if sender == "User" {
        theme::Text::Color([0.7, 0.9, 1.0].into()) // Light blue for user
    } else {
        theme::Text::Color([0.9, 0.9, 0.9].into()) // Light gray for AI
    };

    text(format!("[{}]: {}", sender, message))
        .size(15)
        .style(message_style)
        .into()
}
