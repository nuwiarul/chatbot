use crate::error::AppResult;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct Db {
    pool: sqlx::PgPool,
}

impl Db {
    pub async fn connect(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}

