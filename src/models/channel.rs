use bson::uuid::{Uuid};
use serde::{Deserialize, Serialize};

use super::{Topic, Note};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: Uuid,
    name: String,
    picture_url: Option<String>,
    topics: Vec<Topic>,
    board: Board,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    notes: Vec<Note>
}
