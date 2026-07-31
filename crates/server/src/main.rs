use utils::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    server_lib::run().await
}
