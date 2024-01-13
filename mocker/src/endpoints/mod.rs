mod updaters;

use axum::routing::put;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("interval", put(updaters::update_interval))
        .route("/topic", put(updaters::update_topic))
}
