use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use chrono::{DateTime, Utc};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use sqlx::PgPool;
use time::Duration as TimeDuration;
use uuid::Uuid;

use crate::{configuration::OpenIdClient, AppState};

use super::views::login_button;

const SESSION_MAX_AGE: i64 = 7 * 24 * 60 * 60; // 1 week

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
        .get(oauth_client.user_info_url)
        .bearer_auth(auth_token.access_token().secret())
        .send()
        .await
        .unwrap();
    let user_info = profile.json::<UserInfo>().await.unwrap();

    upsert_user(&user_info, &state.db).await.unwrap();

    let token_max_age = Utc::now() + chrono::Duration::seconds(SESSION_MAX_AGE);

    let session_id = auth_token.access_token().secret();

    upsert_session(&user_info, session_id, token_max_age, &state.db)
        .await
        .unwrap();

    let session_cookie = Cookie::build(("sid", session_id.to_owned()))
        .domain(format!(".{}", state.domain))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(SESSION_MAX_AGE))
        .build();

    (cookie_jar.add(session_cookie), Redirect::to("/"))
}

pub async fn login(Extension(oauth_client): Extension<OpenIdClient>) -> impl IntoResponse {
    login_button(&oauth_client)
}

async fn upsert_session(
    user: &UserInfo,
    session_id: &str,
    expires_at: DateTime<Utc>,
    db: &PgPool,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "INSERT INTO sessions (user_id, session_id, expires_at) VALUES (
                (SELECT ID FROM users WHERE email = $1 LIMIT 1), $2, $3)
                 ON CONFLICT (user_id) DO UPDATE SET
                 session_id = EXCLUDED.session_id,
                 expires_at = EXCLUDED.expires_at",
        &user.email,
        session_id,
        expires_at
    )
    .execute(db)
    .await
    .unwrap();

    Ok(())
}

async fn upsert_user(user: &UserInfo, db: &PgPool) -> Result<(), anyhow::Error> {
    let user_id = Uuid::new_v4();

    sqlx::query!("INSERT INTO users (id, email, created_at) VALUES ($1, $2, $3) ON CONFLICT (email) DO NOTHING",
        user_id,
        user.email,
        Utc::now(),
    )
        .execute(db)
        .await
        .unwrap();

    Ok(())
}
