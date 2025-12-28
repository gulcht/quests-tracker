use std::sync::Arc;

use quests_tracker::{
    config::config_loaders,
    infrastructure::{axum_http::http_serve::start, postgres::postgres_connection},
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loaders::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let postges_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("postgres connection has been established");

    start(Arc::new(dotenvy_env), Arc::new(postges_pool))
        .await
        .expect("Failed to start server.");
}
