use hyper::Result;
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};
use zero2prod::{configuration::get_configuration, startup::run, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = Arc::new(AppState::new(
        PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres."),
    ));
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Could not bind to port.");
    run(listener, connection_pool)?.await
}
