use crate::{
    routes::{health_check, subscribe},
    AppState,
};
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::{server::conn::AddrIncoming, Error, Request};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, MakeRequestUuid, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use uuid::Uuid;

#[derive(Clone)]
struct ZeroRequestId;

impl MakeRequestId for ZeroRequestId {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let state = Arc::new(AppState::new(db_pool));
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
        .with_state(state.clone());
    let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
