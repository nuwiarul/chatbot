mod config;
mod db;
mod error;
mod http;

use crate::config::Config;
use crate::db::Db;
use crate::error::AppResult;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env()?;
    let db = Db::connect(&config.database_url).await?;
    let bind_addr = config.bind_addr();

    let state = http::AppState {
        config: config.clone(),
        db: db.clone(),
    };

    let app = http::create_router(state.clone())
        .layer(cors_layer(&config))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind(bind_addr).await?;
    info!("listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

fn cors_layer(config: &Config) -> CorsLayer {
    let value = config.cors_allow_origins.trim();
    let allow_origin = if value == "*" {
        AllowOrigin::any()
    } else {
        let origins = value
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<axum::http::HeaderValue>())
            .collect::<Result<Vec<_>, _>>()
            .expect("CORS_ALLOW_ORIGINS must be '*' or a comma-separated list of valid origins");
        AllowOrigin::list(origins)
    };

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_headers(Any)
        .allow_methods(Any)
}
