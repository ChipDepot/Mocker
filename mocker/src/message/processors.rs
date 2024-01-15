use anyhow::{Context, Result};
use rand::Rng;
use starduck::SCMessage;
use std::{collections::HashMap, env, str::FromStr};

use super::build_tuples;

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

pub fn process_args() -> Result<HashMap<String, String>> {
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

pub fn process_fixed_values(message: &mut SCMessage, args: &mut HashMap<String, String>) {
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

pub fn process_random_values(message: &SCMessage, args: &mut HashMap<String, String>) -> SCMessage {
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
