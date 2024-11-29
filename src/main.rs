use std::collections::HashMap;

use axum::{http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Router};
use hermes_error::HermesFormat;
use rand::{thread_rng, Rng};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

mod hermes_error;
mod utils;
mod session;
mod extractor_error;

mod user;
mod channel;
mod membership;
mod role;
mod permission;

pub async fn not_implemented_yet() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "not implemented yet chill".to_string()).into_response()
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

        .route("/roles/create", post(not_implemented_yet))
        .route("/roles/delete", post(not_implemented_yet))
        .route("/roles/edit", post(not_implemented_yet))
        .route("/roles/fetch", post(not_implemented_yet))

        .with_state(SqlitePool::connect_with(SqliteConnectOptions::new().filename("db.sqlite3")).await.unwrap())
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
