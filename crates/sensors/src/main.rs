use sensors_lib::cli;
use clap::Parser;
use utils::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = cli::Arguments::try_parse().map_err(Error::new)?;

    sensors_lib::run(args).await
}
