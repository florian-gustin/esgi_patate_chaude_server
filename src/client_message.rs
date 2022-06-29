use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Subscribe {
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StartGame {
    pub(crate) key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ClientMessage {
    Hello,
    StartGame(StartGame),
    Subscribe(Subscribe),
}
