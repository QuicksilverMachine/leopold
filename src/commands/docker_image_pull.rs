use crate::docker;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct DockerImagePull {
    description: String,
    name: String,
    version: String
}

impl DockerImagePull {
    fn image(&self) -> String {
        format!("{}:{}", self.name, &self.version)
    }

    pub async fn execute(&self) {
        let _ = docker::image_pull(&self.image()).await;
        println!("Image pulled");
    }

    #[allow(dead_code)]
    pub async fn undo(&self) {
        docker::image_remove(&self.image(), true).await;
        println!("Image removed");
    }
}
