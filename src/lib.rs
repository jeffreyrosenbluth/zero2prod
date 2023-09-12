pub mod configuration;
pub mod routes;
pub mod startup;
use axum_macros::FromRef;
use sqlx::PgPool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pg_pool: PgPool,
}

impl AppState {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}
