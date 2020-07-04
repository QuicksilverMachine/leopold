use crate::docker;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct DockerEngineVersion {
    description: String,
}

impl DockerEngineVersion {
    pub async fn execute(&self) {
        match docker::version().await {
            Err(error) => eprintln!("Error: {}", error.message),
            Ok(version) => println!("Docker engine version: {}", version),
        }
    }

    #[allow(dead_code)]
    pub async fn revert(&self) {}
}
