use crate::{api::handles, pipeline::ServerGlobalState};
use axum::{
    routing::{get, post},
    Router,
};

#[derive(Default)]
pub struct ApiServer;

impl ApiServer {
    pub async fn start(
        &self,
        addr: String,
        state: ServerGlobalState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("🚀 Starting the server");
        log::info!("🔥 Listening on: {}", addr);
        log::info!("🔧 Press Ctrl+C to stop the server");

        let app = Router::new()
            .route("/", get(|| async { "Welcome to Bubbaloop!" }))
            //.route("/api/v0/stats/whoami", get(handles::stats::whoami))
            .nest(
                "/api/v0/stats",
                Router::new()
                    .route("/whoami", get(handles::stats::get_whoami))
                    .route("/sysinfo", get(handles::stats::get_sysinfo)),
            )
            .nest(
                "/api/v0/streaming",
                Router::new().route(
                    "/image/{channel_id}",
                    get(handles::streaming::get_streaming_image),
                ),
            )
            .route(
                "/api/v0/recording",
                post(handles::recording::post_recording_command),
            )
            .nest(
                "/api/v0/inference",
                Router::new()
                    .route(
                        "/result/{channel_id}",
                        get(handles::inference::get_inference_result),
                    )
                    .route(
                        "/settings",
                        post(handles::inference::post_inference_settings),
                    ),
            )
            .nest(
                "/api/v0/pipeline",
                Router::new()
                    .route("/start", post(handles::pipeline::start_pipeline))
                    .route("/stop", post(handles::pipeline::stop_pipeline))
                    .route("/list", get(handles::pipeline::list_pipelines))
                    .with_state(state.pipeline_store),
            )
            .with_state(state.result_store);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
