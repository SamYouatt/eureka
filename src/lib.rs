use std::sync::{Arc, Mutex};

use axum::{routing::get, serve::Serve, Router};
use domain::idea::Idea;
use features::{
    create_idea::handler::{create_idea, create_idea_page},
    health_check::health_check,
    idea_list::handler::get_ideas,
    view_idea::handler::get_idea,
};
use tower_http::services::ServeDir;

mod domain;
mod features;

#[derive(Clone)]
pub struct AppState {
    ideas: Arc<Mutex<Vec<Idea>>>,
}

pub async fn run() -> Result<Serve<Router, Router>, std::io::Error> {
    let ideas = Arc::new(Mutex::new(generate_seed_data()));

    let state = AppState {
        ideas: ideas.clone(),
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await?;

    Ok(axum::serve(listener, app))
}

fn generate_seed_data() -> Vec<Idea> {
    let mut ideas = vec![];

    ideas.push(Idea::new("This one", "Track your ideas"));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));
    ideas.push(Idea::new("Candle shop", "Electricity is overrated"));
    ideas.push(Idea::new("Tartan paint", "Brighten up your day"));
    ideas.push(Idea::new(
        "Bucket with a hole",
        "Basically a sprikler but with a handle",
    ));

    ideas
}
