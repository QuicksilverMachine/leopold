use std::io::Result;

mod api;
mod server;
mod containers;


#[actix_rt::main]
async fn main() -> Result<()> {
    server::run().await
}
