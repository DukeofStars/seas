use crate::{CloseToken, SharedState};
use squidserver::Task;
use std::time::Duration;
use tracing::info;

fn execute_task(task: &Task) {
    match task {
        Task::Hello(msg) => {
            println!("Hello: {}", msg);
        }
    }
}
