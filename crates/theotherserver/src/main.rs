use anyhow::Context;
use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use tracing::debug;
use tracing::info;
use tracing::instrument;

#[derive(Parser, Debug)]
struct Cli {
    /// The interface to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    interface: String,
    /// The port to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    debug!(?cli);

    info!("Listening on {}:{}", cli.interface, cli.port);

    let app = Router::new()
        .route("/is_prime/:n", get(handle_is_prime))
        .route("/next_prime/:n", get(handle_next_prime));

    let listener = tokio::net::TcpListener::bind((cli.interface.as_str(), cli.port))
        .await
        .context("Failed to bind to interface")?;

    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
}

#[instrument]
async fn handle_is_prime(n: Path<u64>) -> &'static str {
    if thelib::is_prime(n.0) {
        "true"
    } else {
        "false"
    }
}

#[instrument]
async fn handle_next_prime(n: Path<u64>) -> String {
    thelib::next_prime(n.0).to_string()
}
