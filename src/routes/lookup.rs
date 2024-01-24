use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tracing;

#[tracing::instrument]
pub async fn handler(Query(Params { query }): Query<Params>) -> (StatusCode, Json<Response>) {
    (StatusCode::OK, Json(Response { result: query }))
}

#[derive(Deserialize)]
pub struct Params {
    query: String,
}

#[derive(Serialize)]
pub struct Response {
    result: String,
}
