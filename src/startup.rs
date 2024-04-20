use axum::{
    http::Request, routing::{get, post}, serve::Serve, Extension, Router
};
use oauth2::basic::BasicClient;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    services::ServeDir,
    trace::{self, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use uuid::Uuid;

use crate::{
    features::{
        auth::handler::{handle_login_redirect, login}, create_idea::handler::{cancel_idea_form, create_idea, create_idea_page, get_idea_form}, health_check::health_check, idea_list::handler::get_ideas, view_idea::handler::get_idea
    },
    AppState,
};

#[derive(Clone)]
struct MakeRequestWithTracingId;

impl MakeRequestId for MakeRequestWithTracingId {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();
        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    open_id_client: BasicClient,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let state = AppState { db: db_pool };

    let assets_path = std::env::current_dir().unwrap();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            trace::DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_response(
            trace::DefaultOnResponse::new()
                .include_headers(true)
                .level(Level::INFO),
        );

    let request_id_layer = ServiceBuilder::new()
        .set_x_request_id(MakeRequestWithTracingId)
        .layer(trace_layer)
        .propagate_x_request_id();

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/health_check", get(health_check))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .route("/ideas/new/form", get(get_idea_form))
        .route("/ideas/new/cancel", post(cancel_idea_form))
        .route("/ideas/:id", get(get_idea))
        .route("/login", get(login))
        .route("/login/redirect", get(handle_login_redirect))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(request_id_layer)
        .layer(Extension(open_id_client));

    Ok(axum::serve(listener, app))
}
