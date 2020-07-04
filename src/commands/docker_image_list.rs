use crate::docker;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct DockerImageList {
    description: String,
}

impl DockerImageList {
    pub async fn execute(&self) {
        let images = docker::image_list().await;
        match images {
            Err(error) => eprintln!("{:?}", error),
            Ok(images) => {
                for image in images {
                    println!("{}", image.name)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub async fn revert(&self) {}
}
