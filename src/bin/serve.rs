use argh::FromArgs;
use axum::{
    routing::{get, post},
    Router,
};

use bubbaloop::{compute, stats};

// defaults for the server
const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 3000;

#[derive(FromArgs)]
#[argh(description = "Bubbaloop server")]
struct CLIArgs {
    #[argh(option, short = 'h', default = "DEFAULT_HOST.to_string()")]
    /// the host to listen on
    host: String,

    #[argh(option, short = 'p', default = "DEFAULT_PORT")]
    /// the port to listen on
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: CLIArgs = argh::from_env();

    // format the host and port
    let addr = format!("{}:{}", args.host, args.port);

    log::info!("🚀 Starting the server");
    log::info!("🔥 Listening on: {}", addr);
    log::info!("🔧 Press Ctrl+C to stop the server");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to Bubbaloop!" }))
        .route("/api/v0/compute/:mean_std", post(compute::compute_mean_std))
        .route("/api/v0/stats/:whoami", post(stats::whoami));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
