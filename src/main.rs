use std::net::TcpListener;
use newsletter_rs::{startup::run, configuration};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let configs = configuration::get_configuration()
        .expect("could not find any configuration file"); 

    let address = format!("{}:{}", configs.app_host, configs.app_port);
    let listener = TcpListener::bind(address).expect("could not start tcp listener");
    run(listener)?.await
}
