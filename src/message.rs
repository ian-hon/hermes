use std::{collections::HashMap, net::SocketAddr};

use axum::{extract::{ws::{self, WebSocket}, ConnectInfo, Query, State, WebSocketUpgrade}, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

use crate::{extractor_error::ExtractorError, hermes_error::HermesFormat, session::{RawSessionID, Session}, utils, ws_statemachine::SocketContainer, AppState};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub channel_id: i32,
    pub content: String,
    pub author: String,
    pub timestamp: i32
}
impl Message {
    pub async fn fetch(db: &Pool<Sqlite>, id: i32) -> Option<Message> {
        sqlx::query_as::<_, Message>("select * from message where id = $1;")
            .bind(id)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn fetch_from_channel(db: &Pool<Sqlite>, channel_id: i32, amount: i32) -> Vec<Message> {
        sqlx::query_as::<_, Message>("select * from message where channel_id = $1 order by timestamp desc limit $2;")
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
        if Message::fetch(db, id).await.is_none() {
            return;
        }
        sqlx::query("delete from messages where id = $1;")
            .bind(id)
            .execute(db)
            .await.unwrap();
    }
}

pub async fn message_socket_handler(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    WithRejection(Json(session_id), _): WithRejection<Json<RawSessionID>, ExtractorError>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    utils::ws_request_boiler(app_state, query, session_id, vec![
        ("channel_id", HermesFormat::Number)
    ], addr, ws,
    |db, ws_set, s, query, addr, ws| async move {
        let channel_id = utils::from_query("channel_id", &query).parse::<i32>().unwrap();
        match ws_set.get(&channel_id) {
            Some(socket_container) => {
                println!("attempt : {addr}");
                ws.on_upgrade(move |socket|
                    message_socket(
                        s,
                        db,
                        socket_container,
                        socket,
                        addr
                    )
                );
                "ws upgraded".to_string()
            },
            None => {
                return "no channel".to_string()
            }
        }
    }).await
}

async fn message_socket(
    s: Session,
    db: Pool<Sqlite>,
    socket_container: &SocketContainer, // needs to be &mut?
    mut socket: WebSocket,
    address: SocketAddr
) {
    // this socket is linked to one statemachine only
    if socket.send(ws::Message::Ping(vec![42, 42, 42])).await.is_ok() {
        println!("s({address}) : ping");
    } else {
        println!("s({address}) Error : ping");
        return;
    }

    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        sender.send(ws::Message::Text("".to_string()));
    });

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            let t = msg.into_text().unwrap();
            println!("{address}({cnt}) : {t}");
            if t == "quit".to_string() {
                return cnt;
            }
        }
        cnt
    });

    // https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs

    // // We subscribe *before* sending the "joined" message, so that we will also
    // // display it to our client.
    // let mut rx = server_sender.subscribe();

    // // Now send the "joined" message to all subscribers.
    // let msg = format!("{username} joined.");
    // tracing::debug!("{msg}");
    // let _ = server_sender.send(msg);

    // // Spawn the first task that will receive broadcast messages and send text
    // // messages over the websocket to our client.
    // let mut send_task = tokio::spawn(async move {
    //     while let Ok(msg) = rx.recv().await {
    //         // In any websocket error, break loop.
    //         if sender.send(Message::Text(msg)).await.is_err() {
    //             break;
    //         }
    //     }
    // });

    // // Clone things we want to pass (move) to the receiving task.
    // let tx = server_sender.clone();
    // let name = username.clone();

    // // Spawn a task that takes messages from the websocket, prepends the user
    // // name, and sends them to all broadcast subscribers.
    // let mut recv_task = tokio::spawn(async move {
    //     while let Some(Ok(Message::Text(text))) = receiver.next().await {
    //         // Add username before message.
    //         let _ = tx.send(format!("{name}: {text}"));
    //     }
    // });

    // // If any one of the tasks run to completion, we abort the other.
    // tokio::select! {
    //     _ = &mut send_task => recv_task.abort(),
    //     _ = &mut recv_task => send_task.abort(),
    // };

    // // Send "user left" message (similar to "joined" above).
    // let msg = format!("{username} left.");
    // tracing::debug!("{msg}");
    // let _ = server_sender.send(msg);
}