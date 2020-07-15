use std::io::Write;

static ACCESS: &'static str = "ACCESS";
static SERVER: &'static str = "SERVER";
static TASK: &'static str = "TASK";

pub fn configure_logging() {
    std::env::set_var("MY_LOG_STYLE", "auto");
    std::env::set_var("RUST_LOG", "info,actix_web=info");

    env_logger::builder()
        .format(|buffer, record| {
            let mut args = format!("{}", record.args());
            if !args.contains("log_type") {
                args = format!(
                    r#""log_info": "{}", "message": "{}" "#,
                    SERVER,
                    args.escape_default()
                );
            }
            writeln!(buffer, r#"{{"level": "{}", {}}},"#, record.level(), args)
        })
        .init();
}

pub fn server_log_format() -> String {
    format!(
        r#""log_type": "{}", "url": "%U", "user_ip": "%a", "request": "%r", "status": %s, "size": %b, "referer": "%{{Referer}}i", "user_agent": "%{{User-Agent}}i", "time_ms": "%D""#,
        ACCESS
    )
}

pub fn info(message: String) {
    info!(
        r#""log_type": "{}", "message": "{}""#,
        SERVER,
        message.escape_default()
    )
}

pub fn task_info(task_id: String, message: String) {
    info!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK,
        task_id,
        message.escape_default()
    )
}

pub fn task_error(task_id: String, message: String) {
    error!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK,
        task_id,
        message.escape_default()
    )
}
