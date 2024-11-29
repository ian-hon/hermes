use std::collections::HashMap;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use sqlx::{Pool, Sqlite};

use crate::{extractor_error::ExtractorError, hermes_error::{self, HermesError}, permission::{user_permission_check, Permissions}, session::RawSessionID};

pub struct Channel {
    id: i32,
    name: String,
    description: String
}
// create
// delete
// edit
// fetch

// join
// leave
// add
impl Channel {
    pub async fn create(db: Pool<Sqlite>, name: String, description: String, user: String) {
        sqlx::query("insert into channel(name, description, creator) values($1, $2, $3)")
            .bind(name)
            .bind(description)
            .bind(user)
            .execute(&db)
            .await.unwrap();

        // add user membership into this channel
    }

    pub async fn delete(db: Pool<Sqlite>, channel: i32, user: String) {
        if !user_permission_check(db, user, channel, Permissions::ChannelDelete).await {
            
        }
    }
}

pub async fn create(
    State(db): State<Pool<Sqlite>>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    match session_id.into_session(&db).await {
        Ok(s) => {
            let r = hermes_error::check(&query, vec![
                ("name".to_string(), hermes_error::HermesFormat::Key),
                ("description".to_string(), hermes_error::HermesFormat::Unspecified),
            ]);
            match r {
                HermesError::Success => {
                    Channel::create(db, query.get("name").unwrap().clone(), query.get("description").unwrap().clone(), s.user).await;

                    "Success".into_response()
                },
                _ => serde_json::to_string(&r).unwrap().into_response()
            }
        },
        Err(e) => serde_json::to_string(&e).unwrap().into_response()
    }
}
