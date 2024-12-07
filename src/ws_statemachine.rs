use std::{collections::{HashMap, HashSet}, net::SocketAddr, sync::Arc};

use axum::{extract::{ws::{self, WebSocket}, ConnectInfo, Query, State, WebSocketUpgrade}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use futures::{lock::Mutex, sink::SinkExt, stream::StreamExt};

use crate::{channel::Channel, hermes_error::HermesFormat, session::Session, utils, AppState};

use tokio::sync::broadcast;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageSpecies {
    // to be sent to open websockets
    UserParticipation(String, bool), // X joined, left; true -> joined
    Typical(SentMessage),
    Deletion(i32),
    Edit(i32, String),
}

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
    pub image: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SocketContainer {
    pub channel_id: i32,
    pub tx: broadcast::Sender<String>,
    pub users: HashSet<String>
}
impl SocketContainer {
    pub fn contains(&self, u: String) -> bool {
        self.users.contains(&u)
    }

    pub fn add(&mut self, u: String) {
        self.users.insert(u);
    }

    pub fn broadcast(&mut self, m: MessageSpecies) {
        let _ = self.tx.send(serde_json::to_string(&m).unwrap().to_string());
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageWebsocketError {
    Success,

    UserAlreadyConnected,
}

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

    let _ = socket_container.lock().await.clone().broadcast(MessageSpecies::UserParticipation(s.user.clone(), true));

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

                    sqlx::query("insert into message(channel_id, content, author, timestamp) values($1, $2, $3, $4);")
                        .bind(sc.lock().await.channel_id)
                        .bind(serde_json::to_string(&e).unwrap())
                        .bind(user.user.clone())
                        .bind(utils::get_time())
                        .execute(&db)
                        .await.unwrap();
        
                    let _ = sc.clone().lock().await.clone().broadcast(MessageSpecies::Typical(e));
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

    let _ = socket_container.lock().await.clone().broadcast(MessageSpecies::UserParticipation(s.user.clone(), false));
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
