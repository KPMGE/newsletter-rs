use std::net::TcpListener;

use newsletter_rs::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let address = "localhost:8080";
    let listener = TcpListener::bind(address).expect("could not start tcp listener");
    run(listener)?.await
}
