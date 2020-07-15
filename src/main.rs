use std::io::Result;

mod api;
mod commands;
mod configuration;
mod docker;
mod errors;
mod logger;
mod server;
mod tasks;

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> Result<()> {
    logger::configure_logging();
    logger::info("Starting Leopold server".to_string());
    server::run().await
}
