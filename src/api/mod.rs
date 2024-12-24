// api/mod.rss
use axum::{Json, response::IntoResponse};

pub mod router;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


pub async fn root_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
        "message": "you are at home!"
    });

    Json(json_response)
}