use axum::{http::StatusCode, Extension, Json};
use squidserver::Task;
use tracing::info;

use crate::SharedState;

pub async fn root() -> &'static str {
    "Hello world!"
}

pub async fn get_task_list(
    Extension(state): Extension<SharedState>,
) -> Result<Json<Vec<Task>>, String> {
    Ok(Json::from(state.read().unwrap().tasks.clone()))
}

// Inserts a task for the worker threads to complete
pub async fn insert_task(
    Extension(state): Extension<SharedState>,
    Json(param): Json<Task>,
) -> StatusCode {
    info!("Adding task: {:#?}", param);
    let tasks = &mut state.write().unwrap().tasks;
    tasks.push(param);
    StatusCode::OK
}
