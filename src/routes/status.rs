use crate::state::AppState;
use axum::extract::State;
use std::sync::Arc;
use tracing::{self, error};

#[tracing::instrument]
pub async fn handler(State(state): State<Arc<AppState>>) -> String {
    let mut counter = state.count.lock().await;
    error!("Test error!!");
    *counter += 1;
    counter.to_string()
}
