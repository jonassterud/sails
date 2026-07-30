use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    server_lib::run().await
}
