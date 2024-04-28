use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use chrono::Utc;
use sqlx::PgPool;
use time::Duration;

use crate::{domain::user::AppUser, AppState};

use super::auth_error::AuthError;

pub async fn require_session(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let redirect_to_login = || Redirect::to("/login").into_response();

    let (parts, body) = request.into_parts();

    let cookie_jar: PrivateCookieJar =
        PrivateCookieJar::from_headers(&parts.headers, state.cookie_signing_key);

    let Some(session_cookie) = cookie_jar
        .get("sid")
        .map(|cookie| cookie.value().to_owned())
    else {
        return redirect_to_login();
    };

    if let Err(e) = verify_session(&session_cookie, &state.db).await {
        match e {
            AuthError::ExpiredSession => {
                let session_cookie = Cookie::build(("sid", "EXPIRED"))
                    .domain(format!(".{}", state.domain))
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .max_age(Duration::days(-1))
                    .build();

                return (cookie_jar.add(session_cookie), Redirect::to("/login")).into_response();
            }
            _ => return redirect_to_login(),
        };
    };

    match get_user_from_session(&session_cookie, &state.db).await {
        Ok(Some(_)) => {
            let request = Request::from_parts(parts, body);

            next.run(request).await
        }
        Ok(None) => redirect_to_login(),
        Err(_) => {
            // TODO: error message to user here instead
            redirect_to_login()
        }
    }
}

pub async fn verify_session(session_cookie: &str, db: &PgPool) -> Result<(), AuthError> {
    let query = sqlx::query!(
        "SELECT expires_at FROM sessions WHERE session_id = $1 LIMIT 1",
        session_cookie
    )
    .fetch_optional(db)
    .await;

    match query {
        Ok(Some(expires_at)) => {
            if expires_at.expires_at < Utc::now() {
                return Err(AuthError::ExpiredSession);
            }
            return Ok(());
        }
        Ok(None) => Err(AuthError::NoSessionStored),
        Err(e) => {
            tracing::error!("Failed to query session cookie: {:?}", e);
            Err(AuthError::SqlError)
        }
    }
}

pub async fn get_user_from_session(
    session_cookie: &str,
    db: &PgPool,
) -> Result<Option<AppUser>, AuthError> {
    let query = sqlx::query_as!(
            AppUser,
            "SELECT users.email, users.id FROM sessions LEFT JOIN users ON sessions.user_id = users.id WHERE sessions.session_id = $1 LIMIT 1",
            session_cookie,
            )
            .fetch_optional(db);

    match query.await {
        Ok(maybe_user) => Ok(maybe_user),
        Err(e) => {
            tracing::error!("Failed query while finding user of session: {:?}", e);
            Err(AuthError::SqlError)
        }
    }
}
