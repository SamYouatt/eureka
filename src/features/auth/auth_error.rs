use axum::response::IntoResponse;

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
