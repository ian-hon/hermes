use std::{collections::{HashMap, HashSet}, net::SocketAddr, sync::Arc};

use axum::{extract::{ws::{self, WebSocket}, ConnectInfo, Query, State, WebSocketUpgrade}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

use crate::{channel::Channel, extractor_error::ExtractorError, hermes_error::{self, HermesError, HermesFormat}, session::{RawSessionID, Session}, utils, ws_statemachine::SocketContainer, AppState};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    // what users receive
    pub author: String,
    pub content: String,
    pub timestamp: i64,
    pub reply: Option<i32>,
    pub image: Option<String> // source link?
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SentMessage {
    // what users send
    pub content: String,
    pub reply: Option<i32>,
    pub image: Option<String>
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct RawMessage {
    // in the database
    pub id: i32,
    pub channel_id: i32,
    pub content: String,
    pub author: String,
    pub timestamp: i32
}
impl RawMessage {
    pub async fn fetch(db: &Pool<Sqlite>, id: i32) -> Option<RawMessage> {
        sqlx::query_as::<_, RawMessage>("select * from message where id = $1;")
            .bind(id)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn fetch_from_channel(db: &Pool<Sqlite>, channel_id: i32, amount: i32) -> Vec<RawMessage> {
        sqlx::query_as::<_, RawMessage>("select * from message where channel_id = $1 order by timestamp desc limit $2;")
            .bind(channel_id)
            .bind(amount)
            .fetch_all(db)
            .await.unwrap()
    }

    pub async fn send(db: &Pool<Sqlite>, channel_id: i32, content: String, author: String) {
        sqlx::query("insert into messages(channel_id, content, author, timestamp) values($1, $2, $3, $4);")
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
        sqlx::query("delete from messages where id = $1;")
            .bind(id)
            .execute(db)
            .await.unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageWebsocketError {
    Success,

    UserAlreadyConnected,
}

#[axum::debug_handler]
pub async fn message_socket_handler(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // NOTE:
    // RawSessionID cannot be passed through request body
    // passed through query params as a string instead
    utils::ws_request_boiler(app_state, query, vec![("channel_id", HermesFormat::Number)], addr, ws,
    |app_state, s, query, addr, ws| async move {
        let channel_id = utils::from_query("channel_id", &query).parse::<i32>().unwrap();

        let db = app_state.db.clone();

        // i wonder if theres gonna be an error here in the future
        let binding = app_state.ws_set.lock().await.clone();
        let result = binding.get(&channel_id);

        match result {
            Some(socket_container) => {
                let socket_container = socket_container.clone();

                if socket_container.lock().await.contains(s.user.clone()) {
                    return serde_json::to_string(&MessageWebsocketError::UserAlreadyConnected).unwrap().into_response();
                }

                ws.on_upgrade(move |socket|
                    message_socket(s, db, socket_container, socket, addr)
                )
            },
            None => {
                match Channel::fetch_by_id(&db, channel_id).await {
                    Some(c) => {
                        let socket_container = Arc::new(Mutex::new(SocketContainer { channel_id: c.id, tx: broadcast::channel(1024).0, users: HashSet::new() }));
                        app_state.ws_set.lock().await.insert(c.id, socket_container.clone());
                        // maybe an error here sometime too

                        ws.on_upgrade(move |socket|
                            message_socket(s, db, socket_container, socket, addr)
                        )
                    },
                    None => "no channel".to_string().into_response()
                }
            }
        }
    }).await
}

async fn message_socket(
    s: Session,
    db: Pool<Sqlite>,
    socket_container: Arc<Mutex<SocketContainer>>,
    mut socket: WebSocket,
    address: SocketAddr
) {
    // this socket is linked to one statemachine only
    // if socket.send(ws::Message::Ping(vec![112, 111, 110, 103])).await.is_err() {
    //     return;
    // }

    if socket.send(ws::Message::Text("***".to_string())).await.is_err() {
        return;
    }

    socket_container.lock().await.clone().add(s.user.clone());
    let mut rx = socket_container.lock().await.tx.subscribe();

    let _ = socket_container.lock().await.clone().broadcast(format!("{} has connected", s.user));

    let (mut sender, mut receiver) = socket.split();

    let s_ = s.clone();
    let sc_ = socket_container.clone();
    let mut send_task = tokio::spawn(async move {
        // server sends this

        let user = s_.clone();
        let sc = sc_.clone();

        while let Ok(msg) = rx.recv().await {
            let _ = sender.send(ws::Message::Text(msg)).await;
        }
    });

    let s_ = s.clone();
    let sc_ = socket_container.clone();
    let mut recv_task = tokio::spawn(async move {
        // server receives this

        let user = s_.clone();
        let sc = sc_.clone();

        while let Some(Ok(msg)) = receiver.next().await {
            let msg = msg.into_text().unwrap();
            let candidate = serde_json::from_str::<SentMessage>(msg.as_str());

            match candidate {
                Ok(e) => {
                    println!("{e:?}");

                    sqlx::query("insert into message(channel_id, content, author, timestamp) values($1, $2, $3, $4);")
                        .bind(sc.lock().await.channel_id)
                        .bind(msg.clone())
                        .bind(user.user.clone())
                        .bind(utils::get_time())
                        .execute(&db)
                        .await.unwrap();
        
                    let _ = sc.clone().lock().await.clone().broadcast(msg.clone());
                }
                Err(_) => {
                    println!("unable to parse");
                    return;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let _ = socket_container.lock().await.clone().broadcast(format!("{} has disconnected", s.user));
}

pub async fn debug_state(
    State(app_state): State<AppState>
) -> String {
    let guard = app_state.ws_set.lock().await;
    let a = guard.clone();
    for i in a {
        let g = i.1.lock().await.clone();
        println!("{} : {:?}", i.0, g);
    }

    "".to_string()
}
