use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum Task {
    Hello(String),
}

pub type CloseToken = Arc<Mutex<CloseTokenInner>>;

#[derive(Debug)]
pub struct CloseTokenInner {
    pub running: bool,
}

impl CloseTokenInner {
    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.running = true;
    }
}

impl Default for CloseTokenInner {
    fn default() -> Self {
        CloseTokenInner { running: true }
    }
}
