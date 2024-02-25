use crate::firestore::get;
use crate::{error::Error, models::user::User};
use actix_web::web;

pub async fn get_users() -> Result<web::Json<Vec<User>>, Error> {
    let users = get("users").await?;

    Ok(web::Json(users))
}
