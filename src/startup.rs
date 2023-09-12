use crate::{
    routes::{health_check, subscribe},
    AppState,
};
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use hyper::Error;
use std::net::TcpListener;
use std::sync::Arc;

pub fn run(
    listener: TcpListener,
    state: Arc<AppState>,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state.clone());
    let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
