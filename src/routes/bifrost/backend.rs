use axum::Router;
use axum::extract::{Path, State};
use axum::routing::post;
use uuid::Uuid;

use bifrost_api::config::Z2mServer;
use svc::serviceid::ServiceId;

use crate::routes::bifrost::BifrostApiResult;
use crate::routes::extractor::Json;
use crate::server::appstate::AppState;

#[axum::debug_handler]
async fn post_backend_z2m(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(server): Json<Z2mServer>,
) -> BifrostApiResult<Json<Uuid>> {
    log::info!("Adding new z2m backend: {name:?}");

    let mut config = (*state.config()).clone();
    config.z2m.servers.insert(name.clone(), server.clone());
    state.replace_config(config);

    let uuid = state
        .manager()
        .start(ServiceId::instance("z2m", name))
        .await?;

    Ok(Json(uuid))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/z2m/{name}", post(post_backend_z2m))
}
