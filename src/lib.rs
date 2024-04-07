use domain::idea::Idea;
use sqlx::PgPool;

pub mod domain;
pub mod features;
pub mod configuration;
pub mod startup;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}
