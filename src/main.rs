use std::sync::{Arc, Mutex};

use askama_axum::Template;
use axum::{extract::State, response::IntoResponse, routing::{get, post}, Router};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    ideas: Vec<Idea>
}

#[derive(Template)]
#[template(path = "idea-card.html")]
struct IdeaCard {
    title: String,
    tagline: String,
}

impl IdeaCard {
    pub fn from_idea(idea: Idea) -> IdeaCard {
        IdeaCard { title: idea.title, tagline: idea.tagline }
    }
}

#[derive(Clone)]
struct Idea {
    title: String,
    tagline: String,
}

impl Idea {
    pub fn new(title: &str, tagline: &str) -> Idea {
        Idea { title: title.to_string(), tagline: tagline.to_string() } 
    }
}

#[derive(Clone)]
struct AppState {
    ideas: Arc<Mutex<Vec<Idea>>>,
}

#[tokio::main]
async fn main() {
    let seed_idea = Idea::new("First idea", "bosh");

    let ideas = Arc::new(Mutex::new(vec![]));
    ideas.lock().unwrap().push(seed_idea);

    let state = AppState { ideas: ideas.clone() };

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/", post(post_idea))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_ideas(State(state): State<AppState>) -> impl IntoResponse {
    let ideas = state.ideas.lock().unwrap().to_vec();

    IndexTemplate { ideas }
}

async fn post_idea(State(state): State<AppState>) -> impl IntoResponse {
    let new_idea = Idea::new("Random", "cool");

    state.ideas.lock().unwrap().push(new_idea.clone());
    
    IdeaCard::from_idea(new_idea)
}
