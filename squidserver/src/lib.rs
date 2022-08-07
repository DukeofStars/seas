use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageHeader {
    pub size: u16,
}
