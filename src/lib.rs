use std::sync::{Arc, Mutex};

use domain::idea::Idea;

pub mod domain;
pub mod features;
pub mod configuration;
pub mod startup;

#[derive(Clone)]
pub struct AppState {
    ideas: Arc<Mutex<Vec<Idea>>>,
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
