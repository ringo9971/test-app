use crate::error::Error;
use crate::firestore;
use crate::models::chat::{Chat, Message};

pub async fn write_message(room_id: &str, message: Message) -> Result<Chat, Error> {
    let chat = firestore::write_message(room_id, message).await?;

    Ok(chat)
}

pub async fn get_chats(room_id: &str) -> Result<Chat, Error> {
    let chat = firestore::get_chat(room_id).await?;

    Ok(chat)
}
