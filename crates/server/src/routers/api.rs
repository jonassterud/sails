use axum::{
    Extension, Json, Router,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use utils::{Error, database::models};

pub fn router() -> Router {
    Router::new()
        .route("/api/health", get(get_health))
        .route("/api/temperature", get(get_temperature))
        .route("/api/temperature", post(set_temperature))
        .route("/api/battery", get(get_battery))
        .route("/api/battery", post(set_battery))
        .route("/api/humidity", get(get_humidity))
        .route("/api/humidity", post(set_humidity))
        .route("/api/position", get(get_position))
        .route("/api/position", post(set_position))
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Params {
    limit: Option<i32>,
}

async fn get_health(pool: Extension<SqlitePool>) -> Result<String, Error> {
    match pool.acquire().await {
        Ok(_) => Ok(format!("API: OK\nDatabase: OK")),
        Err(error) => Err(Error::INTERNAL_SERVER_ERROR(error)),
    }
}

#[axum::debug_handler]
async fn get_temperature(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<models::Temperature>>, Error> {
    let limit = params.limit.unwrap_or(10);
    let data = sqlx::query_as!(
        models::Temperature,
        "SELECT * FROM temperature SORTED LIMIT ?",
        limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_temperature(
    pool: Extension<SqlitePool>,
    Json(data): Json<models::NewTemperature>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO temperature(timestamp, value)
            VALUES ($1, $2)
        "#,
        timestamp()?,
        data.value
    )
    .execute(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_humidity(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<models::Humidity>>, Error> {
    let limit = params.limit.unwrap_or(10);
    let data = sqlx::query_as!(
        models::Humidity,
        "SELECT * FROM humidity SORTED LIMIT ?",
        &limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_humidity(
    pool: Extension<SqlitePool>,
    Json(data): Json<models::NewHumidity>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO humidity(timestamp, value)
            VALUES ($1, $2)
        "#,
        timestamp()?,
        data.value
    )
    .execute(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_battery(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<models::Battery>>, Error> {
    let limit = params.limit.unwrap_or(10);
    let data = sqlx::query_as!(
        models::Battery,
        "SELECT * FROM battery SORTED LIMIT ?",
        &limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_battery(
    pool: Extension<SqlitePool>,
    Json(data): Json<models::NewBattery>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO battery(timestamp, value)
            VALUES ($1, $2)
        "#,
        timestamp()?,
        data.value
    )
    .execute(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_position(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<models::Position>>, Error> {
    let limit = params.limit.unwrap_or(10);
    let data = sqlx::query_as!(
        models::Position,
        "SELECT * FROM position SORTED LIMIT ?",
        limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_position(
    pool: Extension<SqlitePool>,
    Json(data): Json<models::NewPosition>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO position(timestamp, latitude, longitude)
            VALUES ($1, $2, $3)
        "#,
        timestamp()?,
        data.latitude,
        data.longitude,
    )
    .execute(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn timestamp() -> Result<i64, Error> {
    use std::time::{SystemTime, UNIX_EPOCH};

    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}

impl<S> FromRequestParts<S> for Params
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Query::<Params>::from_request_parts(parts, state).await {
            Ok(query) => Ok(query.0),
            Err(_) => Ok(Self::default()),
        }
    }
}
