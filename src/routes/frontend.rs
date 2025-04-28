use axum::Router;
use bifrost_api::config::AppConfig;

use crate::server::appstate::AppState;

use tower_http::services::{ServeDir, ServeFile};

pub fn router(config: &AppConfig) -> Router<AppState> {
    let frontend_dir = &config.bifrost.frontend_dir;

    /* Serve index.html no matter what, since the frontend might point to paths
     * under "/frontend" */
    let fallback = ServeFile::new(frontend_dir.join("index.html"));

    let serve_dir = ServeDir::new(frontend_dir).fallback(fallback);

    Router::new().fallback_service(serve_dir)
}
