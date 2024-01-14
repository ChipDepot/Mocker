mod getters;
mod updaters;

use axum::routing::{get, put};
use axum::Router;

pub fn updater_router() -> Router {
    Router::new()
        .route("/interval", put(updaters::update_interval))
        .route("/topic", put(updaters::update_message))
        .route("/args", put(updaters::update_args))
}

pub fn getter_router() -> Router {
    Router::new().route("/device", get(getters::get_device_uuid))
}
