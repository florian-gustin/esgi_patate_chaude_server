use serde::{Serialize, Deserialize};
use crate::challenge_message::ChallengeResult;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Subscribe {
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ClientMessage {
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult),
}
