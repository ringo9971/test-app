use crate::models::chat::{Chat, Message, Messages};
use firestore_db_and_auth::{documents, errors::FirebaseError, Credentials, ServiceSession};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FirestoreError {
    #[error("failed get env error")]
    FailedGetEnv,
    #[error("failed get env error")]
    FailedDeserializeCredentials,
    #[error(transparent)]
    FirebaseError(#[from] FirebaseError),
}

async fn connect() -> Result<ServiceSession, FirestoreError> {
    let credentials_json =
        env::var("CREDENTIALS_JSON").map_err(|_| FirestoreError::FailedGetEnv)?;
    let mut credentials: Credentials = serde_json::from_str(&credentials_json)
        .map_err(|_| FirestoreError::FailedDeserializeCredentials)?;
    credentials.compute_secret().await?;
    ServiceSession::new(credentials)
        .await
        .map_err(FirestoreError::FirebaseError)
}

async fn _write_message(
    session: &ServiceSession,
    room_id: &str,
    messages: Messages,
) -> Result<Messages, FirestoreError> {
    documents::write(
        session,
        "chats",
        Some(room_id),
        &messages,
        documents::WriteOptions::default(),
    )
    .await?;

    Ok(messages)
}

pub async fn get_chat(room_id: &str) -> Result<Chat, FirestoreError> {
    let session = connect().await?;

    let res: Messages = documents::read(&session, "chats", room_id).await?;

    Ok(Chat {
        room_id: room_id.to_string(),
        messages: res.messages,
    })
}

pub async fn write_message(room_id: &str, message: Message) -> Result<Chat, FirestoreError> {
    let session = connect().await?;

    let mut messages = if let Ok(chat) = get_chat(room_id).await {
        chat.messages
    } else {
        vec![]
    };

    messages.push(message);

    let res = _write_message(&session, room_id, Messages { messages }).await?;

    Ok(Chat {
        room_id: room_id.to_string(),
        messages: res.messages,
    })
}
