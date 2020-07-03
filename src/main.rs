use std::io::Result;

mod api;
mod docker;
mod commands;
mod server;


#[actix_rt::main]
async fn main() -> Result<()> {
    server::run().await
}
