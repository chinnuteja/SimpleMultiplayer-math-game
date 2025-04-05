use crate::{
    model::{GameResult, PlayerAnswer},
    utils::generate_question,
    ws::Client,
};
use futures::{SinkExt, StreamExt};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use warp::{
    ws::{Message, WebSocket, Ws},
    Rejection, Reply,
};

pub type Clients = Arc<Mutex<HashMap<String, HashMap<String, Client>>>>;
pub type Answers = Arc<Mutex<HashMap<String, Option<i32>>>>;

const MAX_PLAYERS: usize = 4;

pub async fn ws_handler(
    ws: Ws,
    room_id: String,
    player_id: String,
    clients: Clients,
    answers: Answers,
) -> std::result::Result<impl Reply, Rejection> {
    println!(">> ws_handler triggered for Room: {}, Player: {}", room_id, player_id);
    Ok(ws.on_upgrade(move |socket| async move {
        handle_socket(socket, room_id, player_id, clients, answers).await;
    }))
}

async fn handle_socket(
    ws: WebSocket,
    room_id: String,
    player_id: String,
    clients: Clients,
    answers: Answers,
) {
    println!("New client connected: {} in room: {}", player_id, room_id);

    let (mut ws_tx, mut ws_rx) = ws.split();
    let (client_tx, mut client_rx) = mpsc::unbounded_channel();

    let can_join = {
        let mut rooms = clients.lock().unwrap();
        let room = rooms.entry(room_id.clone()).or_insert_with(HashMap::new);
        if room.len() >= MAX_PLAYERS {
            false
        } else {
            room.insert(
                player_id.clone(),
                Client {
                    id: player_id.clone(),
                    sender: client_tx,
                },
            );
            true
        }
    };

    if !can_join {
        let _ = ws_tx.send(Message::text("Room is full!")).await;
        println!("Rejected {}: room {} is full", player_id, room_id);
        return;
    }

    let room_id_clone = room_id.clone();
    let player_id_clone = player_id.clone();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let recv_clients = clients.clone();
    let recv_answers = answers.clone();
    let recv_task = tokio::spawn(async move {
        let mut winner_declared = false;

        let ready = {
            let rooms = recv_clients.lock().unwrap();
            rooms.get(&room_id_clone).map_or(false, |room| room.len() == MAX_PLAYERS)
        };

        if ready {
            let question = generate_question();
            recv_answers.lock().unwrap().insert(room_id_clone.clone(), Some(question.answer));
            let question_json = serde_json::to_string(&question).unwrap();

            println!("\u{1f522} Room {} Question: {}", room_id_clone, question.question);

            let room = recv_clients.lock().unwrap();
            if let Some(clients) = room.get(&room_id_clone) {
                for (_, client) in clients.iter() {
                    let _ = client.sender.send(Message::text(question_json.clone()));
                }
            }
        }

        while let Some(Ok(msg)) = ws_rx.next().await {
            if winner_declared {
                continue;
            }

            if let Ok(text) = msg.to_str() {
                if let Ok(ans) = serde_json::from_str::<PlayerAnswer>(text) {
                    if let Some(actual_answer) = recv_answers.lock().unwrap().get(&room_id_clone).copied().flatten() {
                        if ans.answer == actual_answer {
                            println!("\u{1f3c6} Room {} Winner: {}", room_id_clone, ans.player_id);

                            let result = GameResult {
                                winner: ans.player_id.clone(),
                                correct_answer: actual_answer,
                            };
                            let result_json = serde_json::to_string(&result).unwrap();

                            let room = recv_clients.lock().unwrap();
                            if let Some(clients) = room.get(&room_id_clone) {
                                for (_, client) in clients.iter() {
                                    let _ = client.sender.send(Message::text(result_json.clone()));
                                }
                            }

                            winner_declared = true;
                        }
                    }
                }
            }
        }
    });

    let _ = tokio::join!(send_task, recv_task);

    println!("Client {} disconnected from room {}", player_id, room_id);
    let mut rooms = clients.lock().unwrap();
    if let Some(room) = rooms.get_mut(&room_id) {
        room.remove(&player_id);
        if room.is_empty() {
            rooms.remove(&room_id);
            answers.lock().unwrap().remove(&room_id);
        }
    }
}