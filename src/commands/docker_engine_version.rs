use crate::docker;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerEngineVersion {
    description: String,
}

impl DockerEngineVersion {
    pub async fn execute(&self) {
        let version = docker::version().await;
        println!("Docker engine version: {}", version);
    }
}
