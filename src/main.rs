use std::io::Result;

mod api;
mod commands;
mod configuration;
mod docker;
mod errors;
mod server;
mod tasks;

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> Result<()> {
    server::run().await
}
