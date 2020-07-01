use std::io::Result;

mod api;
mod docker;
mod commands;
pub mod server;


#[actix_rt::main]
async fn main() -> Result<()> {
    server::run().await
}
