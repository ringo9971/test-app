use crate::firestore::{connect, FirestoreError};
use crate::{error::Error, models::user::User};
use actix_web::web;
use firestore_db_and_auth::documents;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpdateUserError {
    #[error("update user error")]
    UpdateUserError,
    #[error("firestore error")]
    FirestoreError(#[from] FirestoreError),
}

pub async fn update_user(
    user_uid: web::Path<String>,
    user: web::Json<User>,
) -> Result<web::Json<User>, Error> {
    let session = connect().await.map_err(UpdateUserError::FirestoreError)?;

    documents::write(
        &session,
        "users",
        Some(user_uid.to_owned()),
        &user,
        documents::WriteOptions::default(),
    )
    .await
    .map_err(|_| UpdateUserError::UpdateUserError)?;

    Ok(user)
}
