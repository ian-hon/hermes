use std::{collections::HashMap, net::SocketAddr};

use axum::{extract::FromRef, http::StatusCode, response::{IntoResponse, Response}, routing::{any, get, post}, Router};
use tokio::sync::broadcast;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rand::{thread_rng, Rng};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

mod hermes_error;
mod utils;
mod session;
mod extractor_error;
mod ws_statemachine;

mod user;
mod channel;
mod membership;
mod role;
mod permission;
mod message;

pub async fn not_implemented_yet() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "not implemented yet chill".to_string()).into_response()
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub ws_set: HashMap<i32, ws_statemachine::SocketContainer>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hermes at your service" }))
        .route("/user/login", post(user::login))
        .route("/user/signup", post(user::signup))

        .route("/channel/create", post(channel::create))
        .route("/channel/delete", post(channel::delete))
        .route("/channel/edit", post(channel::edit))
        .route("/channel/fetch/all", post(channel::fetch_all))

        .route("/roles/create", post(role::create))
        .route("/roles/delete", post(role::delete))
        .route("/roles/edit", post(not_implemented_yet))
        .route("/roles/fetch", post(role::fetch_all))

        .route("/message/send", any(message::message_socket_handler))
        .route("/message/delete", post(not_implemented_yet))
        .route("/message/edit", post(not_implemented_yet))
        .route("/message/fetch", post(not_implemented_yet))

        .with_state(
            AppState {
                db: SqlitePool::connect_with(SqliteConnectOptions::new().filename("db.sqlite3")).await.unwrap(),
                ws_set: HashMap::new()
            }
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
