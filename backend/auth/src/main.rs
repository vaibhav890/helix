use std::io::Result;

use actix_web::{App, HttpServer, web::get};
use helix_authentication::app_config::AppConfig;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();

    info!("starting app");

    let server_config = AppConfig::load().server;
    let workers = server_config.workers;
    let bind_address = server_config.bind_address();

    HttpServer::new(|| App::new().route("/", get().to(|| async { "Hello, World from cloudf!" })))
        .workers(workers)
        .bind(bind_address)?
        .run()
        .await
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,actix_web=info,sqlx=off")),
        )
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .init();
}
