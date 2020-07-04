use std::io::Result;

mod api;
mod docker;
mod configuration;
mod commands;
mod server;
mod tasks;


#[actix_rt::main]
async fn main() -> Result<()> {
    server::run().await
}
