use axum::{
    Router, extract::Path, http::StatusCode,
    response::{IntoResponse, Json}, routing::get
};
use crate::metrics::{self, Summary, Kind};


pub fn register() -> Router {
    Router::new()
        .route("/", get(get_all_metrics))
        .route("/{kind}", get(get_specific_metric))
}

async fn get_all_metrics() -> impl IntoResponse {
    let mut sys = metrics::init().await;
    let summary = Summary::generate(&mut sys);
    (StatusCode::OK, Json(summary))
}

async fn get_specific_metric(Path(kind): Path<metrics::Kind>) -> impl IntoResponse {
    let sys = metrics::init().await;
    let response: serde_json::Value = match kind {
        metrics::Kind::System => {
            let system = metrics::System::generate();
            serde_json::to_value(system).unwrap()
        }
        metrics::Kind::Process => {
            let processes = metrics::Process::generate(&sys);
            serde_json::to_value(processes).unwrap()
        }
        metrics::Kind::Memory => {
            let memory = metrics::Memory::generate(&sys);
            serde_json::to_value(memory).unwrap()
        }
        metrics::Kind::Cpu => {
            let cpu = metrics::Cpu::generate(&sys);
            serde_json::to_value(cpu).unwrap()
        }
        metrics::Kind::Disk => {
            let disk = metrics::Disk::generate(&sys);
            serde_json::to_value(disk).unwrap()
        }
    };
    (StatusCode::OK, Json(response))
}


