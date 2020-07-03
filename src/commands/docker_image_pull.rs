use crate::docker;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerImagePull {
    description: String,
    name: String,
    version: String
}

impl DockerImagePull {
    pub async fn execute(&self) {
        let image = format!("{}:{}", self.name, self.version);
        let _ = docker::image_pull(image).await;
        println!("Image pulled");
    }
}
