use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn register() -> Router {
    return Router::new().route("/", get(health_check));
}

pub async fn health_check() -> impl IntoResponse {
    return (StatusCode::OK, "Server is running");
}
