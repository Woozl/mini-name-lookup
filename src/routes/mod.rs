use axum::{routing::get, Router};

pub mod lookup;
pub mod status;

#[rustfmt::skip]
pub fn create_router() -> Router {
    Router::new()
      .route("/lookup", get(lookup::handler))
      .route("/status", get(status::handler))
}
