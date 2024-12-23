use std::collections::HashMap;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{extractor_error::ExtractorError, hermes_error, membership, permission::{user_permission_check, PermissionError, Permissions}, role, session::RawSessionID, utils, AppState};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub invite: i32,
    pub default_role: i32
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
        let channel_id = sqlx::query("insert into channel(name, description, creator, invite) values($1, $2, $3, $4)")
            .bind(name)
            .bind(description)
            .bind(user.clone())
            .bind(membership::Membership::generate_invite(db).await)
            .execute(db)
            .await.unwrap().last_insert_rowid();

        let role_id = role::Role::create(db, channel_id as i32, "member".to_string(), 65280, i64::MAX, 1).await; // remove all permissions later
        sqlx::query("update channel set default_role = $1 where id = $2;")
            .bind(role_id)
            .bind(channel_id)
            .execute(db)
            .await.unwrap();
        membership::Membership::add_membership(db, user.clone(), channel_id as i32, role_id as i32).await;
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
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session_id, vec![
        ("name", hermes_error::HermesFormat::Unspecified),
        ("description", hermes_error::HermesFormat::Unspecified)
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
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session_id, vec![
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
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session_id, vec![
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
    State(app_state): State<AppState>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, HashMap::new(), session_id, vec![],|db, s, _| async move {
        serde_json::to_string(&Channel::fetch_all(&db, s.user).await).unwrap()
    }).await
}
