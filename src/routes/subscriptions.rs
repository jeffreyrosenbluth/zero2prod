use axum::{body::Body, http::Response, response::IntoResponse, Form};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> impl IntoResponse {
    Response::builder().status(200).body(Body::empty()).unwrap()
}
