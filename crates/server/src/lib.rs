mod routers;

use axum::{Extension, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utils::{Error, database};

pub async fn run() -> Result<(), Error> {
    // Set up tracing subscriber
    const DEFAULT_FILTER: &str = "server=debug,tower_http=debug,axum::rejection=trace";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create and initialize database
    let pool = database::connection::pool(true).await?;
    let connection = pool.acquire().await?;
    database::connection::initialize(connection).await?;

    // Create listener and router
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5050").await?;
    let app = Router::new()
        .merge(routers::web::router())
        .merge(routers::api::router())
        .layer(CorsLayer::very_permissive()) // TODO
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool));

    // Start listening
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
