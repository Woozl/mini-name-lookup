use crate::state::AppState;
use axum::{http::Method, routing::get, Router};
use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub mod lookup;
pub mod status;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Method::GET)
        .allow_origin(Any);

    Router::new()
        .route("/lookup", get(lookup::handler))
        .route("/status", get(status::handler))
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http().on_request(())) // disable request log
        .layer(cors)
}
