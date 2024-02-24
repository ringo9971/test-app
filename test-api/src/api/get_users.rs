use crate::error::Error;
use crate::firestore::connect;
use actix_web::HttpResponse;
use firestore_db_and_auth::documents;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetUsersError {
    #[error("connection error")]
    ConnectionError,
    #[error("firestore error")]
    FirestoreError,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
}

pub async fn get_users() -> Result<HttpResponse, Error> {
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

    Ok(HttpResponse::Ok().body(format!("{:?}", users)))
}
