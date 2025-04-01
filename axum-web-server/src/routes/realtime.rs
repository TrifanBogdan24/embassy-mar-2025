use axum::{
    Router, extract::Path, http::StatusCode,
    response::{IntoResponse, Json}, routing::get
};
use crate::metrics::{self, Summary, Kind};

use std::{convert::Infallible, time::Duration};
use tokio_stream::{wrappers::IntervalStream, StreamExt};


pub fn register() -> Router {
    Router::new().route("/", get(get_metrics_in_real_time))
}

async fn get_metrics_in_real_time() -> impl IntoResponse {
    let mut sys = metrics::init().await;
    let summary = Summary::generate(&mut sys);
    (StatusCode::OK, Json(summary))
}
