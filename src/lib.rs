use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use reqwest::Client;
use sqlx::PgPool;

pub mod configuration;
pub mod domain;
pub mod features;
pub mod startup;
pub mod telemetry;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    http_client: Client,
    cookie_signing_key: Key
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_signing_key.clone()
    }
}
