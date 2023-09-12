use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use hyper::Error;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
