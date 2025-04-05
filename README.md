# 🧠 Simple Multiplayer Math Game

A real-time multiplayer math game built using **Rust**, **Warp**, and **WebSockets**. Players join a room and compete to answer a math question first. The fastest correct answer wins!



## 🚀 Features

- Built with **Rust** using `warp` and `tokio`
- Supports **multiple players** in a room (max 4)
- Real-time **WebSocket communication**
- Each room has its own game session and state
- First player to answer correctly is the **winner**



## 📦 Tech Stack

- **Backend:** Rust, Warp (WebSocket), Tokio (async)
- **Frontend:** HTML + JavaScript
- **Communication:** WebSocket-based real-time messages
- **Room Logic:** Player matchmaking & state sync via shared `Arc<Mutex<_>>`
