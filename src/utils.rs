use anyhow::Result;
use std::{fs::File, io::Read};
use uuid::Uuid;

use crate::Config;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn convert_duration_to_seconds(s: &str) -> Result<usize, &'static str> {
    let (amount, unit) = s.split_at(s.len() - 1);
    let amount: usize = amount.parse().map_err(|_| "Invalid number")?;

    match unit {
        "d" => Ok(amount * 24 * 60 * 60), // Convert days to seconds
        "h" => Ok(amount * 60 * 60),      // Convert hours to seconds
        "m" => Ok(amount * 60),           // Convert minutes to seconds
        "s" => Ok(amount),                // Seconds
        _ => Err("Unsupported time unit"),
    }
}

pub fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn load_config_from_file(file_path: &str) -> Result<Config> {
    // Open the file in read-only mode.
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    serde_json::from_str(&contents).map_err(|err| err.into())
}
