use bson::{Uuid, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    content: String,
    sender: String, // user email
    timestamp: DateTime,
}
