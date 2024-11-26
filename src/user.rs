use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode};
use sqlx::{Pool, Sqlite, Row};
use strum_macros::Display;

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

pub async fn login(State(db): State<Pool<Sqlite>>, Query(params): Query<HashMap<String, String>>) -> Result<String, StatusCode> {
    Ok(User::login(&db, params.get(&"username".to_string()).unwrap().clone(), params.get(&"password".to_string()).unwrap().clone()).await.to_string())
}

pub async fn signup(State(db): State<Pool<Sqlite>>, Query(params): Query<HashMap<String, String>>) -> String {
    User::signup(&db,  params.get(&"username".to_string()).unwrap().clone(), params.get(&"password".to_string()).unwrap().clone()).await.to_string()
}
