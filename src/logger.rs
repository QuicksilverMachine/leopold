use std::io::Write;
use std::time::SystemTime;

static ACCESS: &str = "ACCESS";
static SERVER: &str = "SERVER";
static TASK_META: &str = "TASK_META";
static TASK_OUTPUT: &str = "TASK_OUTPUT";

pub fn configure_logging() {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::builder()
        .format(|buffer, record| {
            let mut args = format!("{}", record.args());
            if !args.contains("log_type") {
                args = format!(
                    r#""log_type": "{}", "message": "{}" "#,
                    SERVER,
                    args.escape_default()
                );
            }
            writeln!(
                buffer,
                r#"{{"time": "{}", "level": "{}", {}}}"#,
                server_time_format(),
                record.level(),
                args
            )
        })
        .init();
}

pub fn server_time_format() -> String {
    format!("{:?}", SystemTime::now())
}

pub fn server_log_format() -> String {
    format!(
        r#""log_type": "{}", "url": "%U", "user_ip": "%a", "request": "%r", "status": %s, "size": %b, "referer": "%{{Referer}}i", "user_agent": "%{{User-Agent}}i", "duration_ms": %D"#,
        ACCESS
    )
}

pub fn info<T: AsRef<str>>(message: T) {
    info!(
        r#""log_type": "{}", "message": "{}""#,
        SERVER,
        message.as_ref().escape_default()
    )
}

pub fn info_task<T1: AsRef<str>, T2: AsRef<str>>(task_id: T1, message: T2) {
    info!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK_OUTPUT,
        task_id.as_ref(),
        message.as_ref().escape_default()
    )
}

pub fn info_task_meta<T1: AsRef<str>, T2: AsRef<str>>(task_id: T1, message: T2) {
    info!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK_META,
        task_id.as_ref(),
        message.as_ref().escape_default()
    )
}

pub fn error_task<T1: AsRef<str>, T2: AsRef<str>>(task_id: T1, message: T2) {
    error!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK_OUTPUT,
        task_id.as_ref(),
        message.as_ref().escape_default()
    )
}

pub fn error_task_meta<T1: AsRef<str>, T2: AsRef<str>>(task_id: T1, message: T2) {
    error!(
        r#""log_type": "{}", "task_id": "{}", "message": "{}""#,
        TASK_META,
        task_id.as_ref(),
        message.as_ref().escape_default()
    )
}
