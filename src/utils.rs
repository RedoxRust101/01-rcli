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
