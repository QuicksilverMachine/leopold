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
        match docker::image_pull(&self.image()).await {
            Err(error) => eprintln!("Error: {:?}", error),
            Ok(_) => println!("Image pulled"),
        }
    }

    #[allow(dead_code)]
    pub async fn revert(&self) {
        match docker::image_remove(&self.image(), true).await {
            Err(error) => eprintln!("Error: {:?}", error),
            Ok(_) => println!("Image removed"),
        }
    }
}
