use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, Row};
use strum_macros::Display;

use crate::hermes_error::HermesError;

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String
}
impl User {
    pub async fn login(db: &Pool<Sqlite>, username: String, password: String) -> AccountResult {
        if !User::username_existance(db, &username).await {
            return AccountResult::UsernameNoExist;
        }

        if sqlx::query("select count(*) from user where username = $1 and password = $2;")
            .bind(username)
            .bind(password)
            .fetch_one(db)
            .await
            .unwrap().get::<i32, usize>(0) < 1 {
            return AccountResult::PasswordWrong
        }

        AccountResult::Success
    }

    pub async fn username_existance(db: &Pool<Sqlite>, username: &String) -> bool {
        sqlx::query("select count(*) from user where username = $1;")
            .bind(username.clone())
            .fetch_one(db)
            .await
            .unwrap().get::<i32, usize>(0) >= 1
    }

    pub async fn signup(db: &Pool<Sqlite>, username: String, password: String) -> AccountResult {
        if User::username_existance(db, &username).await {
            return AccountResult::UsernameExist;
        }

        sqlx::query("insert into user(username, password) values($1, $2);")
            .bind(username)
            .bind(password)
            .execute(db)
            .await.unwrap();

        AccountResult::Success
    }
}

#[derive(Display)]
pub enum AccountResult {
    Success,

    // login
    PasswordWrong,
    UsernameNoExist,

    // signup
    UsernameExist
}

pub async fn login(State(db): State<Pool<Sqlite>>, WithRejection(Json(user_info), _): WithRejection<Json<User>, HermesError>) -> impl IntoResponse {
    User::login(&db, user_info.username, user_info.password).await.to_string()
}

pub async fn signup(State(db): State<Pool<Sqlite>>, WithRejection(Json(user_info), _): WithRejection<Json<User>, HermesError>) -> impl IntoResponse {
    User::signup(&db, user_info.username, user_info.password).await.to_string()
}
