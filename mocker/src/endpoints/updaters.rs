use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::ConnectInfo;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, Router};

use chrono::Duration;
use serde_json::Value;
use starduck::SCMessage;

pub async fn update_interval(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(duration): Extension<Arc<Mutex<Duration>>>,
    Json(new_dur): Json<std::time::Duration>,
) -> Response {
    info!("{method} from {addr}");

    match (duration.lock(), chrono::Duration::from_std(new_dur)) {
        (Ok(mut guard), Ok(dur)) => {
            *guard = dur;
            (StatusCode::OK).into_response()
        }
        (_, Err(_)) => (StatusCode::BAD_REQUEST).into_response(),
        (Err(_), _) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn update_topic(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(message): Extension<Arc<Mutex<SCMessage>>>,
    Json(value): Json<Value>,
) -> Response {
    info!("{method} from {addr}");

    (StatusCode::OK).into_response()
}
