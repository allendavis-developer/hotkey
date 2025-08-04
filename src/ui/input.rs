use crate::message::Message;
use iced::{
    theme,
    widget::{container, text, text_input},
    Element, Length,
};

pub fn build_input_section(input_value: &str) -> Element<Message> {
    text_input("Ask me anything...", input_value)
        .on_input(Message::InputChanged)
        .on_submit(Message::SendPressed)
        .padding(10)
        .width(Length::Fill)
        .into()
}

pub fn build_screenshot_preview() -> Element<'static, Message> {
    container(
        text("🖼️ Screenshot thumbnail preview")
            .size(14)
            .style(theme::Text::Color([0.8, 0.8, 0.8].into())),
    )
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fixed(100.0))
    .style(theme::Container::Box)
    .into()
}
