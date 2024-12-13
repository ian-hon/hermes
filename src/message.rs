use std::{collections::{HashMap, HashSet}, net::SocketAddr, sync::Arc};

use axum::{extract::{ws::{self, WebSocket}, ConnectInfo, Query, State, WebSocketUpgrade}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

use crate::{channel::Channel, extractor_error::ExtractorError, hermes_error::{self, HermesError, HermesFormat}, permission::{self, permission_check, user_permission_check, PermissionError}, session::{RawSessionID, Session}, user, utils, ws_statemachine::{Message, MessageSpecies, SentMessage, SocketContainer}, AppState};

#[derive(FromRow, Serialize, Deserialize)]
pub struct RawMessage {
    // in the database
    pub id: i32,
    pub channel_id: i32,
    pub content: String,
    pub author: String,
    pub timestamp: i32,
    pub edited_timestamp: i32,
}
impl RawMessage {
    pub fn to_message(&self) -> Message {
        let sent_message = serde_json::from_str::<SentMessage>(&self.content).unwrap();

        Message {
            id: self.id,
            author: self.author.clone(),
            content: sent_message.content.clone(),
            timestamp: self.timestamp as i64,
            edited_timestamp: self.edited_timestamp as i64,
            reply: sent_message.reply,
            image: sent_message.image
        }
    }

    pub async fn fetch(db: &Pool<Sqlite>, id: i32) -> Option<RawMessage> {
        sqlx::query_as::<_, RawMessage>("select * from message where id = $1;")
            .bind(id)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn fetch_from_channel(db: &Pool<Sqlite>, channel_id: i32, amount: i32) -> Vec<Message> {
        sqlx::query_as::<_, RawMessage>("select * from message where channel_id = $1 order by timestamp desc limit $2;")
            .bind(channel_id)
            .bind(amount)
            .fetch_all(db)
            .await.unwrap()
            .iter()
            .map(|x| x.to_message())
            .collect::<Vec<Message>>()
    }

    pub async fn send(db: &Pool<Sqlite>, channel_id: i32, content: String, author: String) {
        sqlx::query("insert into message(channel_id, content, author, timestamp) values($1, $2, $3, $4);")
            .bind(channel_id)
            .bind(content)
            .bind(author)
            .bind(utils::get_time())
            .execute(db).await.unwrap();
    }

    pub async fn delete(db: &Pool<Sqlite>, id: i32) {
        if RawMessage::fetch(db, id).await.is_none() {
            return;
        }
        sqlx::query("delete from message where id = $1;")
            .bind(id)
            .execute(db)
            .await.unwrap();
    }

    pub async fn edit(db: &Pool<Sqlite>, id: i32, new_content: String, edited_timestamp: i32) {
        match RawMessage::fetch(db, id).await {
            Some(m) => {
                // no error handling after unwrapping because if there is something wrong, IT SHOULD PANIC
                let mut sent_message = serde_json::from_str::<SentMessage>(&m.content).unwrap();
                sent_message.content = new_content;

                sqlx::query("update message set content = $1, edited_timestamp = $2 where id = $3;")
                    .bind(serde_json::to_string(&sent_message).unwrap())
                    .bind(edited_timestamp)
                    .bind(id)
                    .execute(db)
                    .await.unwrap();
            },
            None => {}
        }
        // content is in json form, very detailed
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageError {
    Success,
    MessageNoExist
}

pub async fn delete(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler_whole(app_state, query, session_id, vec![
        ("message_id", HermesFormat::Number)
    ],
    |app_state, s, query| async move {
        // check if message belongs to self
        // if not, check for delete messages perms

        let message_id = utils::from_query("message_id", &query).parse::<i32>().unwrap();
        let db = app_state.db.clone();

        match RawMessage::fetch(&db, message_id).await {
            Some(m) => {
                if (m.author == s.user) || (permission::user_permission_check(&db, &s.user, m.channel_id, permission::Permissions::MessageDelete).await == PermissionError::Success) {
                    RawMessage::delete(&db, m.id).await;
                    let b = app_state.ws_set.lock().await;
                    match b.get(&m.channel_id) {
                        Some(s) => {
                            s.lock().await.broadcast(MessageSpecies::Deletion(m.id));
                        },
                        // no subscribers to channel, dont need to do anything
                        None => {}
                    }

                    return serde_json::to_string(&MessageError::Success).unwrap().to_string();
                }

                serde_json::to_string(&PermissionError::NoPermission).unwrap().to_string()
            },
            None => serde_json::to_string(&MessageError::MessageNoExist).unwrap().to_string()
        }
    }).await
}

pub async fn edit(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler_whole(app_state, query, session_id, vec![
        ("message_id", HermesFormat::Number),
        ("new_content", HermesFormat::Unspecified)
    ],
    |app_state, s, query| async move {
        // check if message belongs to self
        // if not, check for delete messages perms

        let message_id = utils::from_query("message_id", &query).parse::<i32>().unwrap();
        let db = app_state.db.clone();

        match RawMessage::fetch(&db, message_id).await {
            Some(m) => {
                if s.user != m.author {
                    return serde_json::to_string(&PermissionError::NoPermission).unwrap();
                }

                let new_content = utils::from_query("new_content", &query);
                let edited_timestamp = utils::get_time() as i32;
                RawMessage::edit(&db, m.id, new_content.clone(), edited_timestamp).await;

                let b = app_state.ws_set.lock().await;
                match b.get(&m.channel_id) {
                    Some(s) => {
                        s.lock().await.broadcast(MessageSpecies::Edit(m.id, new_content, edited_timestamp));
                    },
                    // no subscribers to channel, dont need to do anything
                    None => {}
                }

                serde_json::to_string(&MessageError::Success).unwrap().to_string()
            },
            None => serde_json::to_string(&MessageError::MessageNoExist).unwrap().to_string()
        }
    }).await
}

pub async fn fetch(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>
) -> impl IntoResponse {
    utils::request_boiler(app_state, query, session_id, vec![
        ("channel_id", HermesFormat::Number),
        ("amount", HermesFormat::Number)
    ], |db, s, query| async move {
        let channel_id = utils::from_query("channel_id", &query).parse::<i32>().unwrap();
        let amount = utils::from_query("amount", &query).parse::<i32>().unwrap().min(50);
        if user_permission_check(&db, &s.user, channel_id, permission::Permissions::MessageView).await == PermissionError::NoPermission {
            return serde_json::to_string(&PermissionError::NoPermission).unwrap();
        }

        serde_json::to_string(
            &RawMessage::fetch_from_channel(&db, channel_id, amount).await
        ).unwrap()
    }).await
}
