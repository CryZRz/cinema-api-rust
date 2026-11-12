use axum::{response::IntoResponse, Json};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "cinema-api"
    }))
}
