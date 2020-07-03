use std::io::Result;

mod api;
mod docker;
mod parser;
mod commands;
mod server;


#[actix_rt::main]
async fn main() -> Result<()> {
    // TODO: Remove this later
    parser::parse().await;

    server::run().await
}
