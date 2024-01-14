use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::ConnectInfo;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde_json::json;

use starduck::SCMessage;

pub async fn get_device_uuid(
    method: Method,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(message): Extension<Arc<Mutex<SCMessage>>>,
) -> Response {
    info!("{method} from {addr}");

    let device_uuid = message.lock().unwrap().device_uuid.clone();

    let body = json!({
        "device_uuid": device_uuid
    });

    (StatusCode::OK, Json(body)).into_response()
}
