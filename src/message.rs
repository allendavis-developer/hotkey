#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendPressed,
    OpenSettings,

    ShowWindow,
    HideWindow,

}


