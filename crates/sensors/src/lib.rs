pub mod cli;

use std::fs;
use tokio::time;
use tokio::task::JoinSet;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utils::Error;
use utils::database::models;

const API_URL: &str = "http://localhost:5050/api";

pub async fn run(
    args: cli::Arguments,
) -> Result<(), Error> {
    // Set up tracing subscriber
    const DEFAULT_FILTER: &str = "sensors=debug";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(Error::new)?;

    // Set up handles
    let mut handles: JoinSet<Result<(), Error>> = JoinSet::new();
    let client = reqwest::Client::new();

    // Temperature
    if let Some(path) = args.temperature {
        let client = client.clone();
        let mut interval = interval(args.timeout);
        
        handles.spawn(async move {
            tracing::info!("Watching temperature at: {:?}", &path);

            loop {
                interval.tick().await;

                let value = fs::read_to_string(&path).map_err(Error::new)?;
                let value = value.parse::<f64>().map_err(Error::new)?;
    
                client.post(&format!("{API_URL}/temperature"))
                    .json(&models::NewTemperature {
                        value: value,
                    })
                    .send()
                    .await?;
            }
        });
    }

    // Humidity
    if let Some(path) = args.humidity.clone() {
        let client = client.clone();
        let mut interval = interval(args.timeout);

        handles.spawn(async move {
            tracing::info!("Watching humidity at: {:?}", &path);

            loop {
                interval.tick().await;

                let value = fs::read_to_string(&path).map_err(Error::new)?;
                let value = value.parse::<f64>().map_err(Error::new)?;
    
                client.post(&format!("{API_URL}/humidity"))
                    .json(&models::NewHumidity {
                        value: value,
                    })
                    .send()
                    .await?;
            }
        });
    }

    // Battery
    if let Some(path) = args.battery.clone() {
        let client = client.clone();
        let mut interval = interval(args.timeout);


        handles.spawn(async move {
            tracing::info!("Watching battery at: {:?}", &path);

            loop {
                interval.tick().await;

                let value = fs::read_to_string(&path).map_err(Error::new)?;
                let value = value.parse::<f64>().map_err(Error::new)?;
    
                client.post(&format!("{API_URL}/battery"))
                    .json(&models::NewBattery {
                        value: value,
                    })
                    .send()
                    .await?;
            }
        });
    }

    // Position
    if let Some(path) = args.position.clone() {
        let client = client.clone();
        let mut interval = interval(args.timeout);

        handles.spawn(async move {
            tracing::info!("Watching position at: {:?}", &path);

            loop {
                interval.tick().await;

                let value = fs::read_to_string(&path).map_err(Error::new)?;
                let value = value.split(',').map(|v| v.parse::<f64>().map_err(Error::new)).collect::<Result<Vec<f64>, Error>>()?;
                let latitude = *value.get(0).ok_or(Error::new("Missing latitude"))?;
                let longitude = *value.get(0).ok_or(Error::new("Missing longitude"))?;
    
                client.post(&format!("{API_URL}/position"))
                    .json(&models::NewPosition {
                        latitude: latitude,
                        longitude: longitude,
                    })
                    .send()
                    .await?;
            }
        });
    }

    // Await all reads
    handles.join_all().await.into_iter().collect::<Result<Vec<()>, Error>>()?;

    Ok(())
}

fn interval(timeout: u64) -> time::Interval {
    let mut out = time::interval(time::Duration::from_secs(timeout));
    out.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    out
}

fn read_write() {

}
