use axum::{
  Router, http::StatusCode,
  response::IntoResponse, routing::get
};


mod healthcheck;
mod metrics;
mod realtime;

pub fn app() -> Router {
  Router::new()
    .route("/", get(home_page))
    .nest("/healthcheck", healthcheck::register())
    .nest("/metrics", metrics::register())
    .nest("/realtime", realtime::register())
}

pub async fn home_page() -> impl IntoResponse {
  (StatusCode::OK, "This is a system montior HTTP server")
}