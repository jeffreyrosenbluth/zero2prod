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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, state),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(state: State<Arc<AppState>>, form: Form<FormData>) -> impl IntoResponse {
    match insert_subscriber(state, form).await {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, state)
)]
pub async fn insert_subscriber(
    state: State<Arc<AppState>>,
    form: Form<FormData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
