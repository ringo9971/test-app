use crate::error::Error;
use crate::firestore::{connect, FirestoreError};
use actix_web::web;
use firestore_db_and_auth::documents;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeleteUserError {
    #[error("create user error")]
    DeleteUserError,
    #[error("firestore error")]
    FirestoreError(#[from] FirestoreError),
}

pub async fn delete_user(user_uid: web::Path<String>) -> Result<web::Json<()>, Error> {
    let session = connect().await.map_err(DeleteUserError::FirestoreError)?;

    documents::delete(&session, &format!("users/{}", user_uid), true)
        .await
        .map_err(|_| DeleteUserError::DeleteUserError)?;

    Ok(web::Json(()))
}
