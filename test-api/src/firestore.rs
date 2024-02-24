use std::env;

use firestore_db_and_auth::{errors::FirebaseError, Credentials, ServiceSession};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FirestoreError {
    #[error("failed get env error")]
    FailedGetEnv,
    #[error(transparent)]
    FirebaseError(#[from] FirebaseError),
}

pub async fn connect() -> Result<ServiceSession, FirestoreError> {
    let credential_file = env::var("CREDENTIAL_FILE").map_err(|_| FirestoreError::FailedGetEnv)?;
    let cred = Credentials::from_file(&credential_file).await?;
    ServiceSession::new(cred)
        .await
        .map_err(FirestoreError::FirebaseError)
}
