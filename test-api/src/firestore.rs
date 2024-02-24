use std::env;

use firestore_db_and_auth::{errors::FirebaseError, Credentials, ServiceSession};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FirestoreError {
    #[error("failed get env error")]
    FailedGetEnv,
    #[error(transparent)]
    FirebaseError(#[from] FirebaseError),
    #[error("failed get env error")]
    FailedDeserializeCredentials,
}

pub async fn connect() -> Result<ServiceSession, FirestoreError> {
    let credentials_json =
        env::var("CREDENTIALS_JSON").map_err(|_| FirestoreError::FailedGetEnv)?;
    let mut credentials: Credentials = serde_json::from_str(&credentials_json)
        .map_err(|_| FirestoreError::FailedDeserializeCredentials)?;
    credentials.compute_secret().await?;
    ServiceSession::new(credentials)
        .await
        .map_err(FirestoreError::FirebaseError)
}
