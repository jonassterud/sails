use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Timeout between reading a sensor, in seconds
    #[arg(short, long)]
    pub timeout: u64,
    /// Path to temperature sensor
    #[arg(long, value_name = "FILE")]
    pub temperature: Option<PathBuf>,
    /// Path to humidity sensor
    #[arg(long, value_name = "FILE")]
    pub humidity: Option<PathBuf>,
    /// Path to battery sensor
    #[arg(long, value_name = "FILE")]
    pub battery: Option<PathBuf>,
    /// Path to position sensor
    #[arg(long, value_name = "FILE")]
    pub position: Option<PathBuf>,
}
