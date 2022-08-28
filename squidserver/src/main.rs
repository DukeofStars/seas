use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{routing::*, Extension, Router, Server};

use tokio::signal;
use tracing::info;

pub use squidserver::*;

mod routes;
mod worker;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = SharedState::default();

    let app = app(state.clone()).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 5664));
    info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(state.clone()))
        .await
        .unwrap();
}

async fn shutdown_signal(state: SharedState) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }

    info!("Shutting down the server")
}

async fn app(state: SharedState) -> Router {
    Router::new()
        .route("/", get(routes::root))
        .route("/tasks/", get(routes::get_task_list))
        .route("/do", post(routes::do_task))
        .layer(Extension(state))
}

type SharedState = Arc<RwLock<State>>;

#[derive(Debug, Default)]
pub struct State {
    tasks_progress: Vec<Box<dyn TaskProgress>>,
}
