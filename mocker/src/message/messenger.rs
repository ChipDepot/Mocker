use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::Utc;
use paho_mqtt::{ConnectOptionsBuilder, MessageBuilder};

use starduck::SCMessage;
use starduck::WithOffset;

use crate::message;

const DEVICE_MESSAGES: &str = "device-messages";

pub async fn messenger(
    base_scmessage: Arc<Mutex<SCMessage>>,
    interval: Arc<Mutex<Duration>>,
    args: &mut HashMap<String, String>,
) {
    let cli = super::build_mqtt_client(&args);
    let conn_opts = ConnectOptionsBuilder::new().clean_session(true).finalize();

    if let Err(e) = cli.connect(conn_opts) {
        panic!("Unable to connect: {:?}", e);
        // std::process::exit(-1);
    }

    loop {
        let guard = base_scmessage.lock().unwrap().clone();
        let dur = interval.lock().unwrap().clone();

        let mut scmessage = message::process_random_values(&guard, args);
        let now = Utc::now_with_offset();

        scmessage.timestamp = now;

        let message = MessageBuilder::new()
            .topic(DEVICE_MESSAGES)
            .payload(scmessage.to_string())
            .finalize();

        if let Err(e) = cli.publish(message) {
            error!("Could not publish message: {}", e);
            continue;
        }

        info!("Published message at {}", now);

        std::thread::sleep(dur)
    }
}
