use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Temperature {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTemperature {
    pub value: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Humidity {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewHumidity {
    pub value: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Battery {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBattery {
    pub value: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Position {
    pub timestamp: i64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPosition {
    pub latitude: f64,
    pub longitude: f64,
}
