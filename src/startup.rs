use std::sync::{Arc, Mutex};

use axum::{routing::get, serve::Serve, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::{features::{create_idea::handler::{create_idea, create_idea_page}, health_check::health_check, idea_list::handler::get_ideas, view_idea::handler::get_idea}, generate_seed_data, AppState};

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Serve<Router, Router>, std::io::Error> {
    let ideas = Arc::new(Mutex::new(generate_seed_data()));

    let state = AppState {
        ideas: ideas.clone(),
        db: db_pool,
    };

    let assets_path = std::env::current_dir().unwrap();

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/health_check", get(health_check))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .route("/ideas/:id", get(get_idea))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    Ok(axum::serve(listener, app))
}
