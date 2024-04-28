use axum::response::IntoResponse;

#[derive(Debug)]
pub enum AuthError {
    NoSessionCookie,
    NoSessionStored,
    ExpiredSession,
    NoMatchingUserForSession,
    SqlError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
