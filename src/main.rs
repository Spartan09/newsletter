use newsletter::configuration::get_configuration;
use newsletter::startup::Appliction;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let server = Appliction::build(configuration).await?;
    server.run_until_stopped().await?;
    Ok(())
}
