use crate::firestore::update;
use crate::{error::Error, models::user::User};
use actix_web::web;

pub async fn update_user(
    user_uid: web::Path<String>,
    user: web::Json<User>,
) -> Result<web::Json<User>, Error> {
    let user = update("users", &user_uid, user).await?;

    Ok(user)
}
