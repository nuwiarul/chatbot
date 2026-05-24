pub mod handlers;
pub mod middleware;

use crate::config::Config;
use crate::db::Db;
use axum::Router;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Db,
}

pub fn create_router(state: AppState) -> Router<AppState> {
    handlers::router(state)
}
