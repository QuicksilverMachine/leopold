use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde::{Deserialize};

use crate::commands::command::{Command};


#[derive(Deserialize, Debug)]
pub(crate) struct Configuration {
    extends: Vec<String>,
    pub(crate) tasks: HashMap<String,Vec<Command>>,
}

pub(crate) async fn parse_configuration(app: &str) -> Configuration {
    let config_file = format!("config/{}.yaml", app);
    let mut file = File::open(config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let configuration: Configuration = serde_yaml::from_str(&contents).unwrap();
    configuration
}
