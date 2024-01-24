use crate::routes::create_router;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::fmt::format;

mod routes;
mod searcher;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .event_format(format().with_target(false))
        .init();

    let app_state = Arc::new(state::AppState::new());

    let _s = searcher::Searcher::new();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("Listening at {}", listener.local_addr()?);
    axum::serve(listener, create_router(app_state)).await?;

    Ok(())
}
