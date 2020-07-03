use crate::docker;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerImageList {
    description: String,
}

impl DockerImageList {
    pub async fn execute(&self) {
        let images = docker::image_list().await;
        for image in images {
            println!("{}", image.name)
        }
    }
}
