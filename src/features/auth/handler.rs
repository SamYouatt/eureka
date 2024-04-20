use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use chrono::Local;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use time::Duration as TimeDuration;

use crate::AppState;

use super::views::login_button;

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
}

#[derive(Deserialize)]
struct UserInfo {
    email: String,
}

pub async fn login_callback(
    State(state): State<AppState>,
    cookie_jar: PrivateCookieJar,
    Query(auth_request): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
) -> impl IntoResponse {
    let auth_token = oauth_client
        .exchange_code(AuthorizationCode::new(auth_request.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let token_expiration = auth_token
        .expires_in()
        .ok_or(anyhow!("Token did not have expiration time"))
        .unwrap();
    let token_expiration: i64 = token_expiration.as_secs().try_into().unwrap();

    let session_cookie = Cookie::build(("sid", auth_token.access_token().secret().to_owned()))
        .domain(".app.localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(token_expiration))
        .build();

    (cookie_jar.add(session_cookie), Redirect::to("/"))
}

pub async fn login(Extension(oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    login_button(
        oauth_client.client_id(),
        oauth_client
            .redirect_url()
            .expect("Couldn't find redirect url"),
    )
}
