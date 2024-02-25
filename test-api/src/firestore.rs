use firestore_db_and_auth::{documents, errors::FirebaseError, Credentials, ServiceSession};
use futures_util::StreamExt;
use serde::{de::DeserializeOwned, Serialize};
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

async fn _write<T>(
    session: &ServiceSession,
    collection_name: &str,
    document_name: Option<&str>,
    model: &T,
) -> Result<(), FirestoreError>
where
    T: Serialize,
{
    documents::write(
        session,
        collection_name,
        document_name,
        model,
        documents::WriteOptions::default(),
    )
    .await?;

    Ok(())
}

pub async fn get<T>(collection_name: &str) -> Result<Vec<T>, FirestoreError>
where
    T: DeserializeOwned + 'static,
{
    let session = connect().await?;
    let mut stream = documents::list(&session, collection_name);

    let mut models = vec![];
    while let Some(Ok((doc, _metadata))) = stream.next().await {
        models.push(doc);
    }

    Ok(models)
}

pub async fn create<T>(collection_name: &str, model: T) -> Result<T, FirestoreError>
where
    T: Serialize,
{
    let session = connect().await?;

    _write(&session, collection_name, None, &model).await?;

    Ok(model)
}

pub async fn update<T>(
    collection_name: &str,
    document_name: &str,
    model: T,
) -> Result<T, FirestoreError>
where
    T: Serialize,
{
    let session = connect().await?;

    _write(&session, collection_name, Some(document_name), &model).await?;

    Ok(model)
}

pub async fn delete(collection_name: &str, document_name: &str) -> Result<(), FirestoreError> {
    let session = connect().await?;

    documents::delete(
        &session,
        &format!("{}/{}", collection_name, document_name),
        true,
    )
    .await?;

    Ok(())
}
