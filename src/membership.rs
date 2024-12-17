use std::{collections::{HashMap, HashSet}, i32};

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite, Row};

use crate::{channel::Channel, extractor_error::ExtractorError, hermes_error::HermesFormat, session::RawSessionID, utils, AppState};

#[derive(FromRow)]
pub struct Membership {
    pub id: i32,
    pub channel_id: i32,
    pub user: String,
    pub role_id: i32 // -1 represents no role (default)
}
impl Membership {
    pub async fn generate_invite(db: &Pool<Sqlite>) -> i32 {
        let ids: HashSet<i32> = HashSet::from_iter(sqlx::query("select invite from channel;").fetch_all(db).await.unwrap().iter().map(|x| x.get::<i32, usize>(0)).into_iter());
        loop {
            // will this bite me back in the ass?
            // no way hermes will have 2 billion channels right?
            let candidate = utils::async_rng_int(i32::MAX - 1);
            if ids.contains(&candidate) {
                continue;
            }

            return candidate;
        }
    }

    pub async fn use_invite(db: &Pool<Sqlite>, invite: i32, user: String) -> MembershipError {
        match Membership::get_invite(db, invite).await {
            Some(c) => {
                match Membership::fetch_membership(db, user.clone(), c.id).await {
                    Some(_) => MembershipError::MembershipExists,
                    None => {
                        Membership::add_membership(db, user, c.id, c.default_role).await;
                        MembershipError::Success
                    }
                }
            },
            None => MembershipError::InviteNoExist
        }
    }

    pub async fn get_invite(db: &Pool<Sqlite>, invite: i32) -> Option<Channel> {
        sqlx::query_as::<_, Channel>("select * from channel where invite = $1;")
            .bind(invite)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn add_membership(db: &Pool<Sqlite>, user: String, channel_id: i32, role_id: i32) {
        sqlx::query("insert into membership(channel_id, user, role_id) values($1, $2, $3);")
            .bind(channel_id)
            .bind(user)
            .bind(role_id)
            .execute(db)
            .await.unwrap();
    }

    pub async fn fetch_membership(db: &Pool<Sqlite>, user: String, channel_id: i32) -> Option<Membership> {
        sqlx::query_as::<_, Membership>("select * from membership where user = $1 and channel_id = $2;")
            .bind(user)
            .bind(channel_id)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn remove_membership() {

    }
}

#[derive(Serialize, Deserialize)]
pub enum MembershipError {
    Success,

    MembershipExists,
    InviteNoExist,
}

pub async fn add_membership(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session_id, vec![
        ("invite", HermesFormat::BigNumber)
    ], |db, s, query | async move {
        serde_json::to_string(&Membership::use_invite(&db, utils::from_query("invite", &query).parse::<i32>().unwrap(), s.user).await).unwrap().to_string()
    }).await
}
