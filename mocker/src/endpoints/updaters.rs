use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::ConnectInfo;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use starduck::SCMessage;

pub async fn update_interval(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(duration): Extension<Arc<Mutex<Duration>>>,
    Json(new_dur): Json<Duration>,
) -> Response {
    info!("{method} from {addr}");

    match (duration.lock(), new_dur) {
        (Ok(mut guard), dur) => {
            *guard = dur;
            (StatusCode::OK).into_response()
        }
        (Err(_), _) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn update_message(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(_message): Extension<Arc<Mutex<SCMessage>>>,
    Json(_new_message): Json<SCMessage>,
) -> Response {
    info!("{method} from {addr}");

    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn update_args(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(_message): Extension<Arc<Mutex<SCMessage>>>,
    Json(_new_message): Json<Vec<String>>,
) -> Response {
    info!("{method} from {addr}");

    (StatusCode::NOT_IMPLEMENTED).into_response()
}
