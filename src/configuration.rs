use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub openid: OpenIdSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub domain: String,
    pub https: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
            .log_statements(tracing_log::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Disable,
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct OpenIdSettings {
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

#[derive(Clone)]
pub struct OpenIdClient {
    pub client: BasicClient,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

impl OpenIdSettings {
    pub fn build_client(&self, settings: &Settings) -> OpenIdClient {
        let auth_url = AuthUrl::new(self.auth_url.to_owned()).expect("Invalid auth endpoint");
        let token_url = TokenUrl::new(self.token_url.to_owned()).expect("Invalid token endpoint");

        let redirect_url = match settings.application.https {
            true => format!("https://{}/login/redirect", settings.application.domain),
            false => format!(
                "http://{}:{}/login/redirect",
                settings.application.domain, settings.application.port
            ),
        };

        let client = BasicClient::new(
            ClientId::new(self.client_id.expose_secret().into()),
            Some(ClientSecret::new(self.client_secret.expose_secret().into())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        OpenIdClient {
            client,
            auth_url: self.auth_url.to_owned(),
            token_url: self.token_url.to_owned(),
            user_info_url: self.user_info_url.to_owned(),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yml")))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Load settings form env variables if provided
        // e.g. APP_APLICATION__PORT=5001 would set the application port to 5001
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment, use either local or production",
                other
            )),
        }
    }
}
