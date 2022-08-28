use axum::{http::StatusCode, Extension, Json};
use squidserver::Task;
use tracing::info;

use crate::SharedState;

pub async fn root() -> &'static str {
    "Hello world!"
}

pub fn get_task_list(Extension(state): Extension<SharedState>) -> Result<String, String> {
    Ok(state
        .read()
        .unwrap()
        .tasks_progress
        .iter()
        .map(|progress| progress.display())
        .collect())
}

// Inserts a task for the worker threads to complete
pub async fn do_task(
    Extension(state): Extension<SharedState>,
    Json(param): Json<Task>,
) -> StatusCode {
    info!("Adding task: {:#?}", param);
    let tasks = &mut state.write().unwrap().tasks_progress;
    tasks.push(param);
    StatusCode::OK
}
