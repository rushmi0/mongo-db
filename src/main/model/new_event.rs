use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
    pub timestamp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Greeting {
    pub timestamp: u32,
    pub message: String,
}
