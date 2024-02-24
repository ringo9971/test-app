use crate::firestore::connect;
use crate::{error::Error, models::user::User};
use actix_web::web;
use firestore_db_and_auth::documents;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("create user error")]
    CreateUserError,
    #[error("connection error")]
    ConnectionError,
}

pub async fn create_user(user: web::Json<User>) -> Result<web::Json<User>, Error> {
    let session = connect()
        .await
        .map_err(|_| CreateUserError::ConnectionError)?;

    documents::write(
        &session,
        "users",
        Some("service_test"),
        &user,
        documents::WriteOptions::default(),
    )
    .await
    .map_err(|_| CreateUserError::CreateUserError)?;

    Ok(user)
}
