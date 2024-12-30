use axum::extract::State;
use axum::response::{IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// Global store of all pipelines managed by the server
pub type PipelineStore = Arc<Mutex<HashMap<String, PipelineHandle>>>;

/// The current status of a pipeline
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PipelineStatus {
    /// The pipeline is running in the background
    Running,
    /// The pipeline is stopped
    Stopped,
    /// The pipeline has encountered an error
    Error(String),
}

/// An object managing a pipeline
#[derive(Debug)]
pub struct PipelineHandle {
    // a unique identifier for the pipeline
    // TODO: explore using a UUID
    id: String,
    /// the task that the pipeline is running
    handle: std::thread::JoinHandle<()>,
    // the status of the pipeline
    status: PipelineStatus,
    // stop signal
    stop_signal: Arc<AtomicBool>,
}

#[derive(Debug, Serialize)]
struct PipelineInfo {
    id: String,
    thread_name: String,
    status: PipelineStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineStartRequest {
    // the id of the pipeline to start
    pub pipeline_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineStopRequest {
    // the id of the pipeline to stop
    pub pipeline_id: String,
}

// initialize the pipeline store
pub fn init_pipeline_store() -> PipelineStore {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Start a pipeline given its id
pub async fn start_pipeline(
    State(store): State<PipelineStore>,
    Json(request): Json<PipelineStartRequest>,
) -> impl IntoResponse {
    log::debug!("Request to start pipeline: {:?}", request);

    // check if the pipeline id is already in the store
    let pipeline_id = request.pipeline_id;
    let mut pipeline_store = store.lock().expect("Failed to lock pipeline store");

    if pipeline_store.contains_key(&pipeline_id) {
        log::error!("Pipeline {} already exists", pipeline_id);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Pipeline already exists",
            })),
        );
    }

    // create the pipeline handle
    let stop_signal = Arc::new(AtomicBool::new(false));
    let handle = std::thread::spawn({
        let stop_signal = stop_signal.clone();
        let pipeline_id = pipeline_id.clone();
        move || {
            while !stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                log::debug!("Pipeline {} is running", pipeline_id);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            log::debug!("Pipeline {} stopped", pipeline_id);
        }
    });

    // add the pipeline handle to the store
    pipeline_store.insert(
        pipeline_id.clone(),
        PipelineHandle {
            id: pipeline_id,
            handle,
            status: PipelineStatus::Running,
            stop_signal,
        },
    );

    (
        StatusCode::OK,
        Json(json!({
            "message": "Pipeline started",
        })),
    )
}

/// Stop a pipeline given its id
pub async fn stop_pipeline(
    State(store): State<PipelineStore>,
    Json(request): Json<PipelineStopRequest>,
) -> impl IntoResponse {
    let pipeline_id = request.pipeline_id;
    let mut store = store.lock().expect("Failed to lock pipeline store");

    let Some(mut pipeline) = store.remove(&pipeline_id) else {
        log::error!("Pipeline {} not found", pipeline_id);
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": format!("Pipeline with id {} not found", pipeline_id),
            })),
        );
    };

    // stop the pipeline
    pipeline.status = PipelineStatus::Stopped;
    pipeline
        .stop_signal
        .store(true, std::sync::atomic::Ordering::Relaxed);

    if let Err(_e) = pipeline.handle.join() {
        log::error!("Failed to join pipeline thread");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to join pipeline thread",
            })),
        );
    }

    assert!(store.get(&pipeline_id).is_none());

    (
        StatusCode::OK,
        Json(json!({
            "message": "Pipeline stopped",
        })),
    )
}

/// List all pipelines and return their status
pub async fn list_pipelines(State(store): State<PipelineStore>) -> impl IntoResponse {
    let store = store.lock().expect("Failed to lock pipeline store");
    let pipelines = store
        .values()
        .map(|pipeline| PipelineInfo {
            id: pipeline.id.clone(),
            thread_name: pipeline.handle.thread().name().unwrap_or("").to_string(),
            status: pipeline.status.clone(),
        })
        .collect::<Vec<_>>();
    Json(pipelines)
}
