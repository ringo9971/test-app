use crate::firestore::connect;
use crate::{error::Error, models::user::User};
use actix_web::web;
use firestore_db_and_auth::documents;
use futures_util::StreamExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetUsersError {
    #[error("connection error")]
    ConnectionError,
    #[error("firestore error")]
    FirestoreError,
}

pub async fn get_users() -> Result<web::Json<Vec<User>>, Error> {
    let session = connect()
        .await
        .map_err(|_| GetUsersError::ConnectionError)?;

    let mut stream = documents::list(&session, "users");

    let mut users = vec![];

    while let Some(Ok(doc_result)) = stream.next().await {
        let (doc, _metadata) = doc_result;
        let user: User = doc;
        users.push(user);
    }

    Ok(web::Json(users))
}
