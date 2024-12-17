use std::{collections::HashMap, f32::MIN_POSITIVE};

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{channel, extractor_error::ExtractorError, hermes_error::HermesFormat, membership::Membership, permission::{self, PermissionError}, session::RawSessionID, utils, AppState};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub channel_id: i32,

    pub name: String,
    pub colour: i32,

    pub content: i64,
    pub hierarchy: i32
    // (0) has higher hierarchy than (1)
    // role (0) can control role (1)
    // role (1) cannot control role (0)
    // 0 - role
    // 1 - role
}

/*
create
delete
fetch
edit
    - change name
    - change colour
    - change perms
    - change hierarchy
*/

impl Role {
    pub async fn create(db: &Pool<Sqlite>, channel_id: i32, name: String, colour: i32, content: i64, hierarchy: i32) -> i64 {
        if channel::Channel::fetch_by_id(&db, channel_id).await.is_some() {
            return sqlx::query("insert into roles(channel_id, name, colour, content, hierarchy) values($1, $2, $3, $4, $5);")
                .bind(channel_id)
                .bind(name)
                .bind(colour)
                .bind(content)
                .bind(hierarchy)
                .execute(db)
                .await.unwrap().last_insert_rowid();
        }

        -1
    }

    pub async fn hierarchy_check(db: &Pool<Sqlite>, imposer: i32, target: i32) -> bool {
        // imposer = doing action on target
        // target = victim of action
        // action is valid only when imposer hierarchy is higher than target's
        let imposer = Role::fetch(db, imposer).await;
        let target = Role::fetch(db, target).await;

        if imposer.is_none() || target.is_none() {
            return false;
        }

        // the closer to 0, the higher the hierarchy
        imposer.unwrap().hierarchy < target.unwrap().hierarchy
    }

    pub async fn delete(db: &Pool<Sqlite>, id: i32) {
        sqlx::query("delete from roles where id = $1;")
            .bind(id)
            .execute(db)
            .await.unwrap();
    }

    pub async fn fetch_all(db: &Pool<Sqlite>, channel_id: i32) -> Vec<Role> {
        sqlx::query_as::<_, Role>("select * from roles where channel_id = $1;")
            .bind(channel_id)
            .fetch_all(db)
            .await.unwrap()
    }

    pub async fn fetch(db: &Pool<Sqlite>, role_id: i32) -> Option<Role> {
        sqlx::query_as::<_, Role>("select * from roles where id = $1;")
            .bind(role_id)
            .fetch_optional(db)
            .await.unwrap()
    }
}

pub async fn create(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session, vec![
        ("channel_id", HermesFormat::Number),
        ("name", HermesFormat::Unspecified),
        ("colour", HermesFormat::Number),
        ("content", HermesFormat::BigNumber),
        ("hierarchy", HermesFormat::Number)
    ], |db, s, query| async move {
        if permission::user_permission_check(&db, &s.user, utils::from_query("channel_id", &query).parse::<i32>().unwrap(), permission::Permissions::RoleCreate).await != PermissionError::Success {
            return serde_json::to_string(&PermissionError::NoPermission).unwrap()
        }

        Role::create(
            &db,
            utils::from_query("channel_id", &query).parse::<i32>().unwrap(),
            utils::from_query("name", &query),
            utils::from_query("colour", &query).parse::<i32>().unwrap(),
            utils::from_query("content", &query).parse::<i64>().unwrap(),
            utils::from_query("hierarchy", &query).parse::<i32>().unwrap(),
        ).await;
        "Success".to_string()
    }).await
}

pub async fn delete(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session, vec![
        ("role_id", HermesFormat::Number),
    ], |db, s, query| async move {
        let target_role = match Role::fetch(&db, utils::from_query("role_id", &query).parse::<i32>().unwrap()).await {
            Some(r) => { r },
            None => { return serde_json::to_string(&PermissionError::NoPermission).unwrap(); }
        };

        if permission::user_permission_check(&db, &s.user, target_role.channel_id, permission::Permissions::RoleDelete).await != PermissionError::Success {
            return serde_json::to_string(&PermissionError::NoPermission).unwrap();
        }

        let self_role = match Role::fetch(&db, Membership::fetch_membership(&db, s.user, target_role.channel_id).await.unwrap().role_id).await {
            Some(r) => { r },
            None => { return serde_json::to_string(&PermissionError::NoPermission).unwrap(); }
        };

        if self_role.hierarchy < target_role.hierarchy {
            Role::delete(&db, target_role.id).await;
        }
        serde_json::to_string(&PermissionError::NoPermission).unwrap()
    }).await
}

pub async fn fetch_all(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session, vec![
        ("channel_id", HermesFormat::Number),
    ], |db, s, query| async move {
        let channel_id = utils::from_query("channel_id", &query).parse::<i32>().unwrap();
        if Membership::fetch_membership(&db, s.user, channel_id).await.is_none() {
            return serde_json::to_string(&PermissionError::NoPermission).unwrap()
        };

        serde_json::to_string(&Role::fetch_all(&db, channel_id).await).unwrap()
    }).await
}
