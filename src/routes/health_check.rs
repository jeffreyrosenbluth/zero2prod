use axum::{body::Body, http::Response, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    Response::builder().status(200).body(Body::empty()).unwrap()
}
