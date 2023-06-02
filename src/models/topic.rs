use bson::Uuid;
use serde::{Deserialize, Serialize};

use super::{Message, Tag};

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    id: Uuid,
    title: String,
    icon: Option<String>,
    messages: Vec<Message>,
    tags: Vec<Tag>
}
