use axum::{extract::State, http, response::IntoResponse, Form};
use chrono::Utc;
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
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(state: State<Arc<AppState>>, form: Form<FormData>) -> impl IntoResponse {
    match insert_subscriber(state, form).await {
        Ok(_) => http::StatusCode::OK,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
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
