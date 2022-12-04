use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;
use zero2prod::telementry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Could not read configuration");

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;
    startup::run(listener, connection)?.await
}
