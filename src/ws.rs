use warp::ws::Message;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub sender: UnboundedSender<Message>,
}
