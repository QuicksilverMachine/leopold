mod docker_engine;
mod docker_images;
mod docker_containers;


pub async fn run_command(command: &String, args: &Vec<String>) {
    match command.as_str() {
        "docker_image_list" => docker_images::list().await,
        "docker_image_pull" => docker_images::pull(&args[0], &args[1]).await,
        "docker_container_list" => docker_containers::list().await,
        "docker_engine_version" => docker_engine::version().await,
        _ => println!("Command {} not found.", command)
    }
}
