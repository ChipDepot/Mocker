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

    let mut args = message::process_args().unwrap();

    let base_scmessage = Arc::new(Mutex::new(message::build_message(&mut args)));
    let axum_base = Arc::clone(&base_scmessage);
    let dur = Arc::new(Mutex::new(message::build_duration(&args)));
    let axum_dur = Arc::clone(&dur);

    // Spawn messenger in the background
    tokio::spawn(async move { message::messenger(base_scmessage, dur, &mut args).await });

    let app = Router::new()
        .layer(Extension(axum_base))
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
