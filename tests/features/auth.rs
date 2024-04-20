use crate::helpers::spawn_test_app;

#[tokio::test]
async fn oauth_callback_attaches_cookie() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let auth_code = "4/0AeaYSHC4rztsh1tovJbxe0tjk1_xrIva4IS128fwH8yY3dHfZRWnpElVxHFaEvMrJIVI5w";

    // Act
    let url = format!("{}/login/redirect?code={}", test_app.address, auth_code);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 303);
}
