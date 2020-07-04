use crate::docker;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct DockerContainerList {
    description: String,
}

impl DockerContainerList {
    pub async fn execute(&self) {
        let container_list = docker::container_list().await;
        match container_list {
            Err(error) => eprintln!("Error: {:?}", error),
            Ok(containers) => {
                for container in containers {
                    println!("{}", container.name)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub async fn revert(&self) {}
}
