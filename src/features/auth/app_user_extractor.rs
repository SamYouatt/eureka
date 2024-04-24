use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::PrivateCookieJar;

use crate::{domain::user::AppUser, AppState};

use super::{auth_error::AuthError, middleware::get_user_from_session};

// Extractor to get the app user
// Will reject if the user is not already logged in
#[axum::async_trait]
impl FromRequestParts<AppState> for AppUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie_jar: PrivateCookieJar = PrivateCookieJar::from_request_parts(parts, state)
            .await
            .unwrap();

        let Some(session_cookie) = cookie_jar
            .get("sid")
            .map(|cookie| cookie.value().to_owned())
        else {
            return Err(AuthError::NoSessionCookie);
        };

        get_user_from_session(&session_cookie, &state.db)
            .await?
            .ok_or(AuthError::NoMatchingUserForSession)
    }
}
