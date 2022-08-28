use crate::{CloseToken, SharedState};
use rayon::prelude::*;
use squidserver::Task;
use std::time::Duration;
use tracing::info;

pub async fn worker(state: SharedState, close_token: CloseToken) -> Result<(), String> {
    info!("Worker thread started");
    while close_token.lock().await.running {
        if state.read().unwrap().tasks.len() < 1 {
            info!("No tasks: Waiting");
            std::thread::sleep(Duration::from_secs(10))
        }
        poll_tasks(&state).await;
    }
    Ok(())
}

async fn poll_tasks(state: &SharedState) {
    state.read().unwrap().tasks.par_iter().for_each(|task| {
        execute_task(task);
    });
    state.write().unwrap().tasks.clear();
}

fn execute_task(task: &Task) {
    match task {
        Task::Hello(msg) => {
            println!("Hello: {}", msg);
        }
        Task::Refresh => todo!(),
    }
}
