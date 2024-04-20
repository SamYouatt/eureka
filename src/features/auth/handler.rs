use axum::{response::IntoResponse, Extension};
use oauth2::basic::BasicClient;
use serde::Deserialize;

use super::views::login_button;

pub async fn login(Extension(oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    login_button(
        oauth_client.client_id(),
        oauth_client
            .redirect_url()
            .expect("Couldn't find redirect url"),
    )
}
