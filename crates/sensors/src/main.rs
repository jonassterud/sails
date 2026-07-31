use utils::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    sensors_lib::run().await
}
