use crate::docker;


pub(crate) async fn version() {
    let version = docker::version().await;
    println!("Docker engine version: {}", version);
}
