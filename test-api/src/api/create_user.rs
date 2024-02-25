use crate::firestore::create;
use crate::{error::Error, models::user::User};
use actix_web::web;

pub async fn create_user(user: web::Json<User>) -> Result<web::Json<User>, Error> {
    let user = create("users", user).await?;

    Ok(user)
}
