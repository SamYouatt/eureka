use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension};
use oauth2::basic::BasicClient;
use serde::Deserialize;

use super::views::login_button;

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
}

pub async fn handle_login_redirect(Query(auth_request): Query<AuthRequest>, Extension(oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    todo!()
}

pub async fn login(Extension(oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    login_button(
        oauth_client.client_id(),
        oauth_client
            .redirect_url()
            .expect("Couldn't find redirect url"),
    )
}
