use sqlx::PgPool;

pub mod configuration;
pub mod domain;
pub mod features;
pub mod startup;
pub mod telemetry;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}
