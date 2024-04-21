use std::io::Error;

use axum::{
    http::Request, middleware, routing::{get, post}, serve::Serve, Extension, Router
};
use axum_extra::extract::cookie::Key;
use oauth2::basic::BasicClient;
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, PgPool};
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
    configuration::{DatabaseSettings, OpenIdClient, Settings},
    features::{
        auth::{handler::{login, login_callback}, middleware::require_session},
        create_idea::handler::{cancel_idea_form, create_idea, create_idea_page, get_idea_form},
        health_check::health_check,
        idea_list::handler::get_ideas,
        view_idea::handler::get_idea,
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

pub struct Application {
    port: u16,
    server: Serve<Router, Router>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, Error> {
        let db_pool = get_db_pool(&configuration.database);

        let open_id_client = configuration.openid.build_client();

        let app_address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let http_client = Client::new();
        let cookie_signing_key = Key::generate();

        let listener = TcpListener::bind(app_address)
            .await
            .expect("Failed to bind listener");
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            db_pool,
            open_id_client,
            http_client,
            cookie_signing_key,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }
}

pub fn get_db_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.with_db())
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    open_id_client: OpenIdClient,
    http_client: Client,
    cookie_signing_key: Key,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let state = AppState {
        db: db_pool,
        http_client,
        cookie_signing_key,
    };

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

    let protected_router = Router::new()
        .route("/", get(get_ideas))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .route("/ideas/new/form", get(get_idea_form))
        .route("/ideas/new/cancel", post(cancel_idea_form))
        .route("/ideas/:id", get(get_idea))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_session));

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/login", get(login))
        .route("/login/redirect", get(login_callback))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(request_id_layer)
        .layer(Extension(open_id_client));

    Ok(axum::serve(listener, app))
}
