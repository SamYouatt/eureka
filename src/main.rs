use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use domain::idea::Idea;
use features::{
    create_idea::handler::{create_idea, create_idea_page}, health_check::health_check, idea_list::handler::get_ideas, view_idea::handler::get_idea
};
use tower_http::services::ServeDir;

use eureka::run;

mod domain;
mod features;

#[derive(Clone)]
pub struct AppState {
    ideas: Arc<Mutex<Vec<Idea>>>,
}

#[tokio::main]
async fn main() {
    run().await
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
