// api/router.rs
use axum::{
    routing::get,
    Router,
};

use super::{health_checker_handler, root_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api", get(root_handler))
}