use axum::{routing::get, Router};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

mod user;
mod channel;
mod membership;
mod role;
mod permission;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hermes at your service" }))
        .route("/user/login", get(user::login))
        .route("/user/signup", get(user::signup))
        .with_state(SqlitePool::connect_with(SqliteConnectOptions::new().filename("db.sqlite3")).await.unwrap())
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
