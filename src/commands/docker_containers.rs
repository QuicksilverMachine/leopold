use crate::docker;


pub(crate) async fn list() {
    let containers = docker::container_list().await;
    for container in containers {
        println!("{}", container.name)
    }
}
