mod endpoints;
mod message;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate log;

use axum::{Extension, Router};
use tokio::net::TcpListener;

const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    // Start the logger and load the env variables
    env_logger::init();

    let args = message::process_args().unwrap();
    let arc_args = Arc::new(Mutex::new(args));
    let axum_args = Arc::clone(&arc_args);

    let base_scmessage = Arc::new(Mutex::new(message::build_message(
        &mut arc_args.lock().unwrap(),
    )));
    let axum_base = Arc::clone(&base_scmessage);

    let dur = Arc::new(Mutex::new(message::build_duration(
        &mut arc_args.lock().unwrap(),
    )));
    let axum_dur = Arc::clone(&dur);

    // Spawn messenger in the background
    tokio::spawn(async move { message::messenger(base_scmessage, dur, arc_args).await });

    let app = Router::new()
        .nest("/update", endpoints::updater_router())
        .nest("/get", endpoints::getter_router())
        .layer(Extension(axum_base))
        .layer(Extension(axum_args))
        .layer(Extension(axum_dur));

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let tcp_listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        error!("Could not start server: {e}");
        std::process::exit(-1);
    });

    info!("Initializing server at {}", &addr);

    axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap_or_else(|e| {
        error!("Could not start server: {e}");
        std::process::exit(-1);
    });
}
