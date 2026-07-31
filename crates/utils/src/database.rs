use crate::error::Error;
use sqlx::{Sqlite, pool::PoolConnection, sqlite::{SqliteConnectOptions, SqlitePool}};

const PATH: &str = "database/data.db";

/// Create a connection pool to the database, creates the database if it does not already exists.
///
/// # Errors
///
/// This function will return an error if creating a pool fails.
pub async fn pool(create_if_missing: bool) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new().create_if_missing(create_if_missing).filename(PATH);
    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}

/// Run migrations for the database.
/// 
/// # Errors
///
/// This function will return an error if running the migrations fail.
pub async fn initialize(mut connection: PoolConnection<Sqlite>) -> Result<(), Error> {
    sqlx::migrate!("../../database/migrations").run(&mut connection).await?;

    Ok(())
}
