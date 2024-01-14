use std::env;
use std::{collections::HashMap, time::Duration};

use anyhow::{bail, Context, Result};
use chrono::Utc;
use paho_mqtt::Client;
use starduck::SCMessage;
use starduck::WithOffset;

const TOPIC: &str = "topic";
const DEFAULT_TOPIC: &str = "temperatura";

const LOCATION: &str = "location";
const DEFAULT_LOCATION: &str = "laboratorios-pesados";

const STATUS: &str = "status";
const DEFAULT_STATUS: &str = "OK";

const ALERT: &str = "alert";
const DEFAULT_ALERT: bool = false;

const MQTT_PORT: &str = "mqtt_port";
const DEFAULT_MQTT_PORT: i32 = 1883;

const DEVICE_UUID: &str = "DEVICE_UUID";

const INTERVAL: &str = "interval";
const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);

const IP: &str = "ip";
const DEFAULT_IP: &str = "localhost";

const SEPARATOR: char = ':';

pub fn build_tuples(k: &String) -> Result<(String, String)> {
    let mut parts = k.split(SEPARATOR);

    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
        return Ok((key.to_string(), value.to_string()));
    }

    bail!("Invalid tuple: {k}");
}

pub fn build_message(value_map: &mut HashMap<String, String>) -> SCMessage {
    let topic = match value_map.get(TOPIC) {
        Some(s) => s.to_owned(),
        None => {
            warn!("Missing {TOPIC} in args, using default '{DEFAULT_TOPIC}'");
            DEFAULT_TOPIC.to_string()
        }
    };
    value_map.remove(TOPIC);

    let timestamp = Utc::now_with_offset();

    let location = match value_map.get(LOCATION) {
        Some(s) => s.to_owned(),
        None => {
            warn!("Missing {LOCATION} in args, using default '{DEFAULT_LOCATION}'");
            DEFAULT_LOCATION.to_string()
        }
    };
    value_map.remove(LOCATION);

    let mut values = HashMap::new();
    values.insert(LOCATION.to_string(), serde_json::Value::String(location));

    let alert = match value_map.get(ALERT) {
        Some(b) => match b.parse() {
            Ok(k) => k,
            Err(_) => {
                warn!("Invalid bool `{b}` as '{ALERT}' defaulting to '{DEFAULT_ALERT}'");
                DEFAULT_ALERT
            }
        },
        None => {
            warn!("Missing {ALERT} in args, using default '{DEFAULT_ALERT}'");
            DEFAULT_ALERT
        }
    };
    value_map.remove(ALERT);

    let status = match value_map.get(STATUS) {
        Some(k) => k.to_owned(),
        None => {
            warn!("Missing {STATUS} in args, using default '{DEFAULT_STATUS}'");
            DEFAULT_STATUS.to_string()
        }
    };
    value_map.remove(STATUS);

    let device_uuid = env::var(DEVICE_UUID)
        .with_context(|| format!("Missing {DEVICE_UUID} in env vars"))
        .unwrap()
        .parse::<uuid::Uuid>()
        .with_context(|| format!("Could not parse {DEVICE_UUID}"))
        .unwrap();

    let base_scmessage = SCMessage {
        topic,
        device_uuid,
        timestamp,
        values,
        alert,
        status,
    };

    base_scmessage
}

pub fn build_mqtt_client(args: &HashMap<String, String>) -> Client {
    let port = match args.get(MQTT_PORT) {
        Some(k) => match k.parse() {
            Ok(p) => p,
            Err(_) => {
                warn!("Invalid '{k}' as '{MQTT_PORT}' in args. Defaulting to port '{DEFAULT_MQTT_PORT}'");
                DEFAULT_MQTT_PORT
            }
        },
        None => {
            warn!("Missing '{MQTT_PORT}' in args using default port '{DEFAULT_MQTT_PORT}'");
            DEFAULT_MQTT_PORT
        }
    };

    let ip = env::var(IP).unwrap_or_else(|_| {
        warn!("Missing env var '{IP}', defaulting to '{DEFAULT_IP}'");
        DEFAULT_IP.to_owned()
    });

    let url = format!("tcp://{ip}:{port}");

    Client::new(url)
        .with_context(|| "Could not build MQTT client")
        .unwrap()
}

pub fn build_duration(args: &HashMap<String, String>) -> Duration {
    match args.get(INTERVAL) {
        Some(dur) => match dur.parse::<u64>() {
            Ok(k) => Duration::from_secs(k),
            Err(_) => {
                warn!("Invalid '{dur}' as '{INTERVAL}' in args. Defaulting to port '{DEFAULT_INTERVAL:?}'");
                DEFAULT_INTERVAL
            }
        },
        None => {
            warn!("Missing '{INTERVAL}' in args using default interval '{DEFAULT_INTERVAL:?}'");
            DEFAULT_INTERVAL
        }
    }
}
