use maud::{html, Markup};

use crate::configuration::OpenIdClient;

pub fn login_button(open_id_client: &OpenIdClient) -> Markup {
    let url = format!(
        "{}?response_type=code&client_id={}&scope=openid%20email&redirect_uri={}",
        open_id_client.auth_url,
        open_id_client.client.client_id().as_str(),
        open_id_client
            .client
            .redirect_url()
            .expect("Couldn't find open id client redirect")
            .as_str()
    );

    html! {
        a href=(url) { "Log in with Google" }
    }
}
