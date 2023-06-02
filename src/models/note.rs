use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: String,
    title: String,
    content: String,
    // TODO change this to HexColor
    color: String,
    x: i32,
    y: i32,
    z: i32,
    width: i32,
    height: i32,
}
