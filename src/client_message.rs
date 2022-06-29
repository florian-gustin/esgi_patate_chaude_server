use serde::{Serialize, Deserialize};
use crate::challenge_message::ChallengeResult;

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
