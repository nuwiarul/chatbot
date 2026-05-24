use crate::error::{AppError, AppResult};
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub llama_base_url: String,
    pub api_key: String,
    pub cors_allow_origins: String,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        let host = get_env("HOST").unwrap_or_else(|| "127.0.0.1".to_string());
        let port = get_env("PORT")
            .as_deref()
            .unwrap_or("8080")
            .parse::<u16>()
            .map_err(|_| AppError::Config("PORT must be a valid u16"))?;

        let database_url =
            get_env("DATABASE_URL").ok_or(AppError::Config("DATABASE_URL is required"))?;

        let llama_base_url =
            get_env("LLAMA_BASE_URL").ok_or(AppError::Config("LLAMA_BASE_URL is required"))?;

        let api_key = get_env("API_KEY").unwrap_or_else(|| "dev-change-me".to_string());
        let cors_allow_origins = get_env("CORS_ALLOW_ORIGINS").unwrap_or_else(|| "*".to_string());

        Ok(Self {
            host,
            port,
            database_url,
            llama_base_url,
            api_key,
            cors_allow_origins,
        })
    }

    pub fn bind_addr(&self) -> SocketAddr {
        // We keep it simple; if host isn't an IP, let bind fail loudly.
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("HOST:PORT must be a valid SocketAddr")
    }
}

fn get_env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}
