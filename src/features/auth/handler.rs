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

use crate::{configuration::OpenIdClient, AppState};

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
    Extension(oauth_client): Extension<OpenIdClient>,
) -> impl IntoResponse {
    let auth_token = oauth_client
        .client
        .exchange_code(AuthorizationCode::new(auth_request.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let profile = state
        .http_client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(auth_token.access_token().secret())
        .send()
        .await
        .unwrap();

    let _profile = profile.json::<UserInfo>().await.unwrap();

    // 1 week
    let session_max_age = 7 * 24 * 60 * 60;
    let _token_max_age = Local::now().naive_utc() + chrono::Duration::seconds(session_max_age);

    let session_cookie = Cookie::build(("sid", auth_token.access_token().secret().to_owned()))
        .domain(".localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(session_max_age))
        .build();

    // TODO: insert the user email into the user table if necessary

    // TODO: insert the session token in the sessions table with its expiry for the user

    (cookie_jar.add(session_cookie), Redirect::to("/"))
}

pub async fn login(Extension(oauth_client): Extension<OpenIdClient>) -> impl IntoResponse {
    login_button(
        oauth_client.client.client_id(),
        oauth_client
            .client
            .redirect_url()
            .expect("Couldn't find redirect url"),
    )
}
