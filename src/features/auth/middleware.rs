use axum::{
    extract::{FromRequest, FromRequestParts, Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::PrivateCookieJar;
use sqlx::PgPool;

use crate::{domain::user::AppUser, AppState};

pub enum AuthError {
    NoSessionCookie,
    NoMatchingUserForSession,
    SqlError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

// Extractor to get the app user
// Will reject if the user is not already logged in
#[axum::async_trait]
impl FromRequest<AppState> for AppUser {
    type Rejection = AuthError;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let (mut parts, _body) = req.into_parts();

        let cookie_jar: PrivateCookieJar = PrivateCookieJar::from_request_parts(&mut parts, state)
            .await
            .unwrap();

        let Some(session_cookie) = cookie_jar
            .get("sid")
            .map(|cookie| cookie.value().to_owned())
        else {
            return Err(AuthError::NoSessionCookie);
        };

        get_user_from_session(&session_cookie, &state.db).await?.ok_or(AuthError::NoMatchingUserForSession)
    }
}

pub async fn require_session(
    State(state): State<AppState>,
    cookie_jar: PrivateCookieJar,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let redirect_to_login = || {
        Redirect::to("/login").into_response()
    };

    let Some(session_cookie) = cookie_jar
        .get("sid")
        .map(|cookie| cookie.value().to_owned())
    else {
        return redirect_to_login();
    };

    match get_user_from_session(&session_cookie, &state.db).await {
        Ok(Some(_)) => {
            next.run(request).await
        }
        Ok(None) => {
            redirect_to_login()
        }
        Err(_) => {
            // TODO: error message to user here instead
            redirect_to_login()
        }
    }
}

async fn get_user_from_session(
    session_cookie: &str,
    db: &PgPool,
) -> Result<Option<AppUser>, AuthError> {
    let query = sqlx::query_as!(
            AppUser,
            "SELECT users.email FROM sessions LEFT JOIN users ON sessions.user_id = users.id WHERE sessions.session_id = $1 LIMIT 1",
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
