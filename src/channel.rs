use std::collections::HashMap;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{extractor_error::ExtractorError, hermes_error, permission::{user_permission_check, PermissionError, Permissions}, session::RawSessionID, utils};

#[derive(FromRow, Serialize, Deserialize)]
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
    pub async fn fetch_by_id(db: &Pool<Sqlite>, channel: i32) -> Option<Channel> {
        sqlx::query_as::<_, Channel>("select * from channel where id = $1")
            .bind(channel)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn create(db: &Pool<Sqlite>, name: String, description: String, user: String) {
        sqlx::query("insert into channel(name, description, creator) values($1, $2, $3)")
            .bind(name)
            .bind(description)
            .bind(user)
            .execute(db)
            .await.unwrap();

        // create default role
        // add user membership into this channel
    }

    pub async fn delete(db: &Pool<Sqlite>, channel: i32) {
        if Channel::fetch_by_id(db, channel).await.is_some() {
            sqlx::query("delete from channel where id = $1;")
                .bind(channel)
                .execute(db)
                .await.unwrap();
        }
    }

    pub async fn edit(db: &Pool<Sqlite>, channel: i32, name: String, description: String) {
        if Channel::fetch_by_id(db, channel).await.is_some() {
            sqlx::query("update channel set name = $1, description = $2 where id = $3;")
                .bind(name)
                .bind(description)
                .bind(channel)
                .execute(db)
                .await.unwrap();
        }
    }

    pub async fn fetch_all(db: &Pool<Sqlite>, user: String) -> Vec<Channel> {
        sqlx::query_as::<_, Channel>("select * from channel where id in (select channel_id from membership where user = $1);")
            .bind(user)
            .fetch_all(db)
            .await.unwrap()
    }
}

pub async fn create(
    State(db): State<Pool<Sqlite>>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(db, query, session_id, vec![
        ("channel_id", hermes_error::HermesFormat::Number),
    ], |db, s, query| async move {
        Channel::create(
            &db,
            utils::from_query("name", &query),
            utils::from_query("description", &query),
            s.user
        ).await;

        "Success".to_string()
    }).await
}

pub async fn delete(
    State(db): State<Pool<Sqlite>>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(db, query, session_id, vec![
        ("channel_id", hermes_error::HermesFormat::Number),
    ], |db, s, query| async move {
        let channel_id = utils::from_query("channel_id", &query).parse::<i32>().unwrap();
        match user_permission_check(&db, &s.user, channel_id, Permissions::ChannelDelete).await {
            PermissionError::Success => {
                Channel::delete(&db, channel_id).await;
                "Success".to_string()
            },
            p => serde_json::to_string(&p).unwrap()
        }
    }).await
}

pub async fn edit(
    State(db): State<Pool<Sqlite>>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(db, query, session_id, vec![
        ("channel_id", hermes_error::HermesFormat::Number),
        ("name", hermes_error::HermesFormat::Unspecified),
        ("description", hermes_error::HermesFormat::Unspecified)
    ],
    |db, s, query| async move {
        let channel_id = query.get(&"channel_id".to_string()).unwrap().parse::<i32>().unwrap();
        match user_permission_check(&db, &s.user, channel_id, Permissions::ChannelEdit).await {
            PermissionError::Success => {
                Channel::edit(
                    &db,
                    channel_id,
                    utils::from_query("name", &query),
                    utils::from_query("description", &query)
                ).await;
                "Success".to_string()
            },
            p => serde_json::to_string(&p).unwrap()
        }
    }).await
}

pub async fn fetch_all(
    State(db): State<Pool<Sqlite>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(db, HashMap::new(), session_id, vec![],|db, s, _| async move {
        serde_json::to_string(&Channel::fetch_all(&db, s.user).await).unwrap()
    }).await
}
