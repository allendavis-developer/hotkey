use crate::message::Message;
use crate::ui;
use iced::{executor, Application, Command, Element, Theme, window};

pub struct AssistantApp {
    pub input_value: String,
    pub messages: Vec<(String, String)>, // (sender, message)
}

impl Application for AssistantApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            AssistantApp {
                input_value: String::new(),
                messages: vec![
                    ("User".into(), "Why isn't this formula working?".into()),
                    ("AI".into(), "Looks like cell B5 has a syntax error.".into()),
                    ("AI".into(), "Press the hotkey again to share more context.".into()),
                ],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "AI Screen Assistant".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowWindow => window::change_mode(window::Mode::Windowed),
            Message::HideWindow => window::change_mode(window::Mode::Hidden),

            Message::InputChanged(val) => {
                self.input_value = val;
                Command::none()
            }
            Message::SendPressed => {
                if !self.input_value.trim().is_empty() {
                    self.messages.push(("User".into(), self.input_value.trim().to_string()));

                    // Simulated AI response
                    self.messages.push((
                        "AI".into(),
                        "Analyzing... (This would be replaced with AI output)".into(),
                    ));
                    self.input_value.clear();
                }
                Command::none()
            }
            Message::OpenSettings => {
                println!("⚙️ Open settings clicked (not implemented yet)");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        ui::build_main_view(self)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
