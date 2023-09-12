use axum::{body::Body, extract::State, http::Response, response::IntoResponse, Form};
use chrono::Utc;
use hyper::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(state: State<Arc<AppState>>, form: Form<FormData>) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.pg_pool)
    .await
    {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap(),
    }
}
