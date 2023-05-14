use newsletter_rs::configuration;
use newsletter_rs::startup::Application;
use newsletter_rs::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter_rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configs = configuration::get_configuration().expect("could not read configuration file!");
    let app = Application::build(configs).await?;
    app.run_until_stopped().await?;

    Ok(())
}
