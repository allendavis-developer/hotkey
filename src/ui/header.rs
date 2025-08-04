use crate::message::Message;
use iced::{
    theme,
    widget::{button, row, text},
    Alignment, Element, Length,
};

pub fn build_header() -> Element<'static, Message> {
    row![
        text("🤖 AI Assistant")
            .size(22)
            .style(theme::Text::Color([0.95, 0.95, 0.95].into())),

        iced::widget::Space::with_width(Length::Fill),
        
        button("⚙️ Settings")
            .on_press(Message::OpenSettings)
            .style(theme::Button::Secondary),
    ]
    .padding(10)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}
