use rand::Rng;
use crate::model::MathQuestion;

pub fn generate_question() -> MathQuestion {
    let a = rand::thread_rng().gen_range(10..100);
    let b = rand::thread_rng().gen_range(10..100);
    MathQuestion {
        question: format!("What is {} + {}?", a, b),
        answer: a + b,
    }
}
