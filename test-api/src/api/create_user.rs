use crate::firestore::{connect, FirestoreError};
use crate::{error::Error, models::user::User};
use actix_web::web;
use firestore_db_and_auth::documents;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("create user error")]
    CreateUserError,
    #[error("firestore error")]
    FirestoreError(#[from] FirestoreError),
}

pub async fn create_user(user: web::Json<User>) -> Result<web::Json<User>, Error> {
    let session = connect().await.map_err(CreateUserError::FirestoreError)?;

    documents::write(
        &session,
        "users",
        Option::None::<String>,
        &user,
        documents::WriteOptions::default(),
    )
    .await
    .map_err(|_| CreateUserError::CreateUserError)?;

    Ok(user)
}
