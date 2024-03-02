use crate::{
    config::FirestoreConfig,
    models::chat::{Chat, Message, Messages},
};
use firestore_db_and_auth::{documents, errors::FirebaseError, ServiceSession};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FirestoreError {
    #[error(transparent)]
    FirebaseError(#[from] FirebaseError),
}

async fn connect(firestore_config: &FirestoreConfig) -> Result<ServiceSession, FirestoreError> {
    let mut credentials = firestore_config.credentials.to_owned();
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

pub async fn get_chat(
    firestore_config: &FirestoreConfig,
    room_id: &str,
) -> Result<Chat, FirestoreError> {
    let session = connect(firestore_config).await?;

    let res: Messages = documents::read(&session, "chats", room_id).await?;

    Ok(Chat {
        room_id: room_id.to_string(),
        messages: res.messages,
    })
}

pub async fn write_message(
    firestore_config: &FirestoreConfig,
    room_id: &str,
    message: Message,
) -> Result<Chat, FirestoreError> {
    let session = connect(firestore_config).await?;

    let mut messages = if let Ok(chat) = get_chat(firestore_config, room_id).await {
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
