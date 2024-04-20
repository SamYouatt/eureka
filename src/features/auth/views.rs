use maud::{html, Markup};

pub fn login_button(client_id: &str, redirect_uri: &str) -> Markup {
    let endpoint = "https://accounts.google.com/o/oauth2/v2/auth";
    let url = format!(
        "{}?response_type=code&client_id={}&scope=openid%20email&redirect_uri={}",
        endpoint, client_id, redirect_uri
    );

    html! {
        a href=(url) { "Log in with Google" }
    }
}
