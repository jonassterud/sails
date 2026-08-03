use axum::{
    Extension, Json, Router, async_trait,
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
    value: Option<i32>,
    latitude: Option<i32>,
    longitude: Option<i32>,
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
    Query(params): Query<Params>,
) -> Result<(), Error> {
    let value = params.value.ok_or(Error::BAD_REQUEST("Missing value"))?;
    let _ = sqlx::query!(
        "INSERT INTO temperature (timestamp, value) VALUES (?, ?)",
        0,
        value
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
        limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_humidity(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<(), Error> {
    let value = params.value.ok_or(Error::BAD_REQUEST("Missing value"))?;
    let _ = sqlx::query!(
        "INSERT INTO humidity (timestamp, value) VALUES (?, ?)",
        0,
        value
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
        limit
    )
    .fetch_all(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

async fn set_battery(
    pool: Extension<SqlitePool>,
    Query(params): Query<Params>,
) -> Result<(), Error> {
    let value = params.value.ok_or(Error::BAD_REQUEST("Missing value"))?;
    let _ = sqlx::query!(
        "INSERT INTO battery (timestamp, value) VALUES (?, ?)",
        0,
        value
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
    Query(params): Query<Params>,
) -> Result<(), Error> {
    let latitude = params
        .latitude
        .ok_or(Error::BAD_REQUEST("Missing latitude"))?;
    let longitude = params
        .longitude
        .ok_or(Error::BAD_REQUEST("Missing longitude"))?;
    let _ = sqlx::query!(
        "INSERT INTO position (timestamp, latitude, longitude) VALUES (?, ?, ?)",
        0,
        latitude,
        longitude,
    )
    .execute(&pool.0)
    .await
    .map_err(Error::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

#[async_trait]
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
