use axum::extract::State;
use rand::Rng;
use sqlx::{prelude::FromRow, Pool, Sqlite, Row};

use crate::utils;

const SESSION_LENGTH: u32 = 5 * 3 * 4; // fffff-fffff-fffff
const SESSION_EXPIRY: i64 = 3600 * 24; // 24 hours after no activity

#[derive(FromRow)]
pub struct Session {
    pub id: i64,
    pub user: String,
    pub last_used: i64
}

pub async fn get_session_id(db: &Pool<Sqlite>, user: String) -> String {
    let sessions = sqlx::query_as::<_, Session>("select * from session where user = $1 and last_used > $2;")
        .bind(user.clone())
        .bind(utils::get_time() - SESSION_EXPIRY)
        .fetch_optional(db)
        .await.unwrap();

    println!("{}", utils::get_time());

    match sessions {
        Some(s) => format!("{:x}", s.id),
        None => {
            let id = generate_id(db).await;
            sqlx::query("insert into session values($1, $2, $3);")
                .bind(id.clone())
                .bind(user)
                .bind(utils::get_time())
                .execute(db)
                .await.unwrap();
            format!("{:x}", id)
        }
    }
}

async fn generate_id(db: &Pool<Sqlite>) -> i64 {
    // cannot just bitshift, can be reversible
    /*
    or can you?
    let i = last row id

    let a = ENV_1 << i;
    let b = a << ENV_2;
    let c = ENV3 >> b;

    possible?
    */
    let ids = sqlx::query("select id from session;")
        .fetch_all(db).await.unwrap().into_iter().map(|x| x.get(0)).collect::<Vec<i64>>();

    let mut rng = rand::thread_rng();
    loop { // lets see how long it takes an inf loop to bite my back in the ass
        let candidate = rng.gen_range(0..=(2i64.pow(SESSION_LENGTH)));

        if ids.contains(&candidate) {
            continue;
        }

        return candidate;
    }
}

pub async fn test(State(db): State<Pool<Sqlite>>) {
    sqlx::query("insert into session values($1, $2, $3);")
        .bind(16i128.pow(15) as i64)
        .bind("lorem")
        .bind(0)
        .execute(&db)
        .await.unwrap();
}
