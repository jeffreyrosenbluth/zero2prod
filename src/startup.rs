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
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let state = Arc::new(AppState::new(db_pool));
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .with_state(state.clone());
    let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
