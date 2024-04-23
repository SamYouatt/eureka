use axum::{response::IntoResponse, Extension};
use maud::{html, Markup};

use crate::{configuration::OpenIdClient, domain::page::page};

pub async fn handle_get_login(
    Extension(oauth_client): Extension<OpenIdClient>,
) -> impl IntoResponse {
    page(login_button(&oauth_client))
}

fn login_button(open_id_client: &OpenIdClient) -> Markup {
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
        div class="flex w-full h-full justify-center items-center" {
            div class="flex flex-col px-6 py-4 gap-4 bg-white rounded-md border-2 border-slate-200 dark:bg-slate-800 dark:border-slate-600" {
                p class="text-xl font-medium text-slate-700 dark:text-slate-100" { "Get started with Eureka..." }
                hr class="border-dashed border-slate-200 dark:border-slate-600" ;
                a href=(url)
                    class="flex justify-center items-center px-4 py-2 gap-2 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-200 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors duration-100"
                    {
                        img class="w-6 h-6" src="https://www.svgrepo.com/show/475656/google-color.svg" alt="google logo";
                        span { "Log in with Google " }
                    }
            }
        }
    }
}
