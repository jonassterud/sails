use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn router() -> Router {
    Router::new()
        .nest_service("/scripts", ServeDir::new("crates/www/scripts"))
        .nest_service("/styles", ServeDir::new("crates/www/styles"))
        .nest_service("/assets", ServeDir::new("crates/www/assets"))
        .nest_service("/vendor", ServeDir::new("crates/www/vendor"))
        .route_service("/", ServeFile::new("crates/www/pages/index.html"))
}
