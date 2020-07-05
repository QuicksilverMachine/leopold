use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::commands::Command;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub extends: Vec<String>,
    pub tasks: HashMap<String, Vec<Command>>,
}

pub async fn parse_configuration(app: &str) -> Configuration {
    let config_file = format!("config/{}.yaml", app);
    let mut file = File::open(config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let configuration: Configuration = serde_yaml::from_str(&contents).unwrap();
    configuration
}
