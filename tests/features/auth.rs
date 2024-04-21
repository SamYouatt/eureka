use wiremock::{matchers::{method, path}, Mock, ResponseTemplate};

use crate::helpers::spawn_test_app;

#[derive(serde::Serialize)]
struct UseInfoResponse {
    email: String,
}

#[derive(serde::Serialize)]
struct AuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl AuthTokenResponse {
    fn new() -> Self {
        Self {
            access_token: "test_access_token".into(),
            token_type: "basic".into(),
            expires_in: 3600,
        }
    }
}

#[tokio::test]
async fn oauth_callback_attaches_cookie() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    // These routes are mocked and not the real routes used by Google
    let auth_token_response = ResponseTemplate::new(200).set_body_json(AuthTokenResponse::new());
    let _mock_token_exchange = Mock::given(path("/token"))
        .and(method("POST"))
        .respond_with(auth_token_response)
        .named("OpenId auth token exchange")
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    let user_info_response = ResponseTemplate::new(200).set_body_json(UseInfoResponse { email: "test@test.com".into() });
    let _mock_user_info = Mock::given(path("/user"))
        .and(method("GET"))
        .respond_with(user_info_response)
        .named("OpenId user info")
        .expect(1)
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    let auth_code = "test_auth_code";

    // Act
    let url = format!("{}/login/redirect?code={}", test_app.address, auth_code);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert!(response.cookies().find(|cookie| cookie.name() == "sid").is_some());
}
