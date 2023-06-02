use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    id: String,
    name: String,
    color: String,
    icon: Option<String>
}
