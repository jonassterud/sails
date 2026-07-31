use utils::Error;
use anyhow::anyhow;
use axum::{Extension, Router, routing::get};
use sqlx::SqlitePool;

pub fn router() -> Router {
    Router::new().route("/api/health", get(get_health))
}

async fn get_health(pool: Extension<SqlitePool>) -> Result<String, Error> {
    if let Ok(_) = pool.acquire().await {
        Ok(format!("API: OK\nDatabase: OK"))
    } else {
        Err(Error::INTERNAL_SERVER_ERROR(anyhow!(
            "failed connecting to database"
        )))
    }
}
