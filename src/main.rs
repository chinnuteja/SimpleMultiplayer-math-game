mod handler;
mod ws;
mod model;
mod utils;

use crate::handler::{ws_handler, Clients, Answers};
use std::{collections::HashMap, convert::Infallible, sync::{Arc, Mutex}};
use warp::Filter;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let answers: Answers = Arc::new(Mutex::new(HashMap::new()));

    let clients_filter = with_clients(clients.clone());
    let answers_filter = with_answers(answers.clone());

    let ws_route = warp::path!("ws" / String / String)
        .and(warp::ws())
        .and(warp::any().map(move || clients.clone()))
        .and(warp::any().map(move || answers.clone()))
        .and_then(|room_id, player_id, ws: warp::ws::Ws, clients, answers| async move {
            ws_handler(ws, room_id, player_id, clients, answers).await
        });

    let routes = ws_route.with(warp::cors().allow_any_origin());

    println!("🚀 Rust WebSocket server running at ws://localhost:8000/ws/{{room_id}}/{{player_id}}");

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_answers(answers: Answers) -> impl Filter<Extract = (Answers,), Error = Infallible> + Clone {
    warp::any().map(move || answers.clone())
}