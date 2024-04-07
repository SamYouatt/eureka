use axum::{routing::get, serve::Serve, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::{
    features::{
        create_idea::handler::{create_idea, create_idea_page},
        health_check::health_check,
        idea_list::handler::get_ideas,
        view_idea::handler::get_idea,
    },
    AppState,
};

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let state = AppState { db: db_pool };

    let assets_path = std::env::current_dir().unwrap();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/health_check", get(health_check))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .route("/ideas/:id", get(get_idea))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(trace_layer);

    Ok(axum::serve(listener, app))
}
