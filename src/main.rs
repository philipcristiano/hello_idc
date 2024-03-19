use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use clap::Parser;
use maud::{html, DOCTYPE};
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;

use tower_cookies::{Cookie, CookieManagerLayer, Cookies, Key};


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    bind_addr: String,
    #[arg(short, long, default_value = "oidc.toml")]
    config_file: String,
    #[arg(short, long, value_enum, default_value = "INFO")]
    log_level: tracing::Level,
    #[arg(long, action)]
    log_json: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct AppConfig {
    auth: service_conventions::oidc::OIDCConfig,
}
#[derive(Clone, Debug)]
struct AppState {
    auth: service_conventions::oidc::AuthConfig,
}

impl From<AppConfig> for AppState {
    fn from(item: AppConfig) -> Self {
        let auth_config = service_conventions::oidc::AuthConfig{
            oidc_config: item.auth,
            post_auth_path: "/user".to_string(),
            scopes: vec!("profile".to_string(), "email".to_string())
        };
        AppState {
            auth: auth_config
        }
    }
}
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // initialize tracing

    let args = Args::parse();

    service_conventions::tracing::setup(args.log_level);

    let config_file_error_msg = format!("Could not read config file {}", args.config_file);
    let config_file_contents = fs::read_to_string(args.config_file).expect(&config_file_error_msg);

    let app_config: AppConfig =
        toml::from_str(&config_file_contents).expect("Problems parsing config file");
    let app_state: AppState = app_config.into();

    let oidc_router = service_conventions::oidc::router(app_state.auth.clone());
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/user", get(user_handler))
        .nest("/oidc", oidc_router)
        .with_state(app_state.auth.clone())
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr: SocketAddr = args.bind_addr.parse().expect("Expected bind addr");
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// basic handler that responds with a static string
async fn root() -> Response {
    html! {
       (DOCTYPE)
            p { "Welcome!"}
            a href="/oidc/login" { "Login" }
    }
    .into_response()
}

async fn user_handler(user: Option<service_conventions::oidc::OIDCUser>) -> Response {
    if let Some(user) = user {
        html! {
         (DOCTYPE)
              p { "Welcome! " ( user.id)}
              @if let Some(name) = user.name {
                  p{ ( name ) }
              }
              @if let Some(email) = user.email {
                  p{ ( email ) }
              }
              h3 { "scopes" }
              ul {
                @for scope in &user.scopes {
                    li { (scope) }
                }
              }
              h3 { "groups" }
              ul {
                @for group in &user.groups {
                    li { (group) }
                }
              }

              a href="/oidc/login" { "Login" }
        }
        .into_response()
    } else {

        html! {
         (DOCTYPE)
            p { "Welcome! You need to login" }
            a href="/oidc/login" { "Login" }
        }.into_response()
    }
}
