use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::commands::Command;

#[derive(Deserialize, Debug)]
pub struct AppConfiguration {
    pub extends: Vec<String>,
    pub tasks: HashMap<String, Vec<Command>>,
}

pub async fn app_configuration(app: &str) -> Result<AppConfiguration, Box<dyn Error>> {
    let config_file = format!("config/apps/{}.yaml", app);
    let mut file = File::open(config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    match serde_yaml::from_str(&contents) {
        Ok(app_config) => Ok(app_config),
        Err(error) => Err(Box::new(error)),
    }
}
