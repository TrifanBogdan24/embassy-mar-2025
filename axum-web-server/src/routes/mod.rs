use axum::Router;
use axum::routing::get;

mod healthcheck;

pub fn app() -> Router {
    Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/healthcheck", get(healthcheck::health_check))
}
