mod app;
mod message;
mod ui;
mod screenshot;

use app::AssistantApp;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    AssistantApp::run(Settings::default())
}
