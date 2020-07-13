use std::io::Write;

pub static SERVER_LOG_FORMAT: &'static str = "%a %r %s %b %{Referer}i %{User-Agent}i %T";

pub async fn configure_logging() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();
}

pub fn task_info(task_id: String, message: String) {
    info!("[{}] {}", task_id, message)
}

pub fn task_error(task_id: String, message: String) {
    error!("[{}] {}", task_id, message)
}
