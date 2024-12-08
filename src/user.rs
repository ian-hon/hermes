use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, Row};
use strum_macros::Display;

use crate::{extractor_error::ExtractorError, role::Role, session};

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
            .bind(username.clone())
            .bind(password)
            .fetch_one(db)
            .await
            .unwrap().get::<i32, usize>(0) < 1 {
            return AccountResult::PasswordWrong
        }

        AccountResult::Success(session::Session::get_session_id(db, username).await)
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
            .bind(username.clone())
            .bind(password)
            .execute(db)
            .await.unwrap();

        AccountResult::Success(session::Session::get_session_id(db, username).await)
    }
}

#[derive(Display, Serialize, Deserialize)]
pub enum AccountResult {
    Success(String),

    // login
    PasswordWrong,
    UsernameNoExist,

    // signup
    UsernameExist
}

pub async fn login(
    State(db): State<Pool<Sqlite>>,
    WithRejection(Json(user_info), _): WithRejection<Json<User>, ExtractorError>
) -> impl IntoResponse {
    // Success(string)
    // PasswordWrong
    // UsernameNoExist

    // extractor errors
    
    serde_json::to_string(&User::login(&db, user_info.username, user_info.password).await).unwrap()
}

pub async fn signup(State(db): State<Pool<Sqlite>>, WithRejection(Json(user_info), _): WithRejection<Json<User>, ExtractorError>) -> impl IntoResponse {
    // Success(String)
    // UsernameExist

    // extractor errors

    // TODO: input sanitization for this
    serde_json::to_string(&User::signup(&db, user_info.username, user_info.password).await).unwrap()
}
