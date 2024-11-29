use std::{collections::HashMap, future::Future, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::State, response::IntoResponse};
use sqlx::{Pool, Sqlite};

use crate::{hermes_error::{self, HermesError, HermesFormat}, session::{RawSessionID, Session}};

pub fn get_time() -> i64 {
    // epoch unix, in seconds
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards (???)")
        .as_secs() as i64
}

pub fn from_query(k: &str, q: &HashMap<String, String>) -> String {
    return urlencoding::decode(q.get(&k.to_string()).unwrap().clone().as_str()).unwrap().to_string()
}

pub async fn request_boiler<F, Fut>(
    db: Pool<Sqlite>,
    query: HashMap<String, String>,
    session_id: RawSessionID,

    hermes_check: Vec<(&str, HermesFormat)>,
    
    func: F) -> impl IntoResponse
where F: Fn(Pool<Sqlite>, Session, HashMap<String, String>) -> Fut, Fut: Future<Output = String>,
    {
    match session_id.into_session(&db).await {
        Ok(s) => {
            match hermes_error::check(&query, hermes_check) {
                HermesError::Success => func(db, s, query).await.into_response(),
                r => serde_json::to_string(&r).unwrap().into_response()
            }
        },
        Err(e) => serde_json::to_string(&e).unwrap().into_response()
    }
}
