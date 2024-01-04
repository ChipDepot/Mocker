use std::collections::HashMap;
use std::env;
use std::str::FromStr;

#[macro_use]
extern crate log;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

use starduck::SCMessage;
use starduck::WithOffset;

use chrono::Utc;
use uuid::Uuid;

const TOPIC: &str = "topic";
const DEFAULT_TOPIC: &str = "temperatura";

const LOCATION: &str = "location";
const DEFAULT_LOCATION: &str = "laboratorios-pesados";

const STATUS: &str = "status";
const DEFAULT_STATUS: &str = "OK";

const ALERT: &str = "alert";
const DEFAULT_ALERT: bool = false;

const SEPARATOR: char = ':';

#[derive(Serialize, Deserialize)]
enum ValueType {
    Random(i32, i32),
    Fixed(i32),
}

impl FromStr for ValueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("fixed(") && s.ends_with(')') {
            let inner_value = s["fixed(".len()..s.len() - 1].parse().map_err(|_| ())?;
            Ok(ValueType::Fixed(inner_value))
        } else if s.starts_with("random(") && s.ends_with(')') {
            let inner_values: Vec<i32> = s["random(".len()..s.len() - 1]
                .split(',')
                .map(|part| part.trim().parse().map_err(|_| ()))
                .collect::<Result<Vec<i32>, ()>>()?;

            if inner_values.len() == 2 {
                Ok(ValueType::Random(inner_values[0], inner_values[1]))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn build_tuples(k: &String) -> Result<(String, String)> {
    let mut parts = k.split(SEPARATOR);

    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
        return Ok((key.to_string(), value.to_string()));
    }

    bail!("Invalid tuple: {k}");
}

fn process_args() -> Result<HashMap<String, String>> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .iter()
        .map(build_tuples)
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| "Could not build messages from args")?;

    let mut result = HashMap::new();
    for (key, value) in args {
        result.insert(key, value);
    }

    Ok(result)
}

fn build_message(device_uuid: Uuid, value_map: &mut HashMap<String, String>) -> SCMessage {
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
                warn!("Invalid bool `{b}` defaulting to '{DEFAULT_ALERT}'");
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

    SCMessage {
        topic,
        device_uuid,
        timestamp,
        values,
        alert,
        status,
    }
}

fn process_fixed_values(message: &mut SCMessage, args: &mut HashMap<String, String>) {
    let result = args
        .iter()
        .filter(|&(_, value)| value.contains("fixed"))
        .collect::<Vec<_>>();

    for (key, value) in result {
        let val = match ValueType::from_str(value) {
            Ok(ValueType::Fixed(k)) => k.into(),
            _ => {
                warn!("Could not process '{key}' as a fixed value");
                continue;
            }
        };

        message.values.insert(key.clone(), val);
    }
}

fn process_random_values(message: SCMessage, args: &mut HashMap<String, String>) -> SCMessage {
    let result = args
        .iter()
        .filter(|&(_, value)| value.contains("random"))
        .collect::<Vec<_>>();

    let mut new_message = message.clone();
    let mut rng = rand::thread_rng();

    for (key, value) in result {
        let val = match ValueType::from_str(value) {
            Ok(ValueType::Random(min, max)) => rng.gen_range(min..=max).into(),
            _ => {
                warn!("Could not process '{key}' as a random value");
                continue;
            }
        };

        new_message.values.insert(key.clone(), val);
    }

    new_message
}

fn main() {
    // Start the logger and load the env variables
    env_logger::init();

    let mut args = process_args().unwrap();
    let mut message = build_message(Uuid::new_v4(), &mut args);

    process_fixed_values(&mut message, &mut args);

    println!("{}", process_random_values(message, &mut args));
}
