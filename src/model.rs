use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MathQuestion {
    pub question: String,
    pub answer: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerAnswer {
    pub player_id: String,
    pub answer: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameResult {
    pub winner: String,
    pub correct_answer: i32,
}
