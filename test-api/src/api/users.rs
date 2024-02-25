use crate::firestore::{create, delete, get, update};
use crate::{error::Error, models::user::User};
use actix_web::web;

pub async fn create_user(user: web::Json<User>) -> Result<web::Json<User>, Error> {
    let user = create("users", user).await?;

    Ok(user)
}

pub async fn get_users() -> Result<web::Json<Vec<User>>, Error> {
    let users = get("users").await?;

    Ok(web::Json(users))
}

pub async fn update_user(
    user_uid: web::Path<String>,
    user: web::Json<User>,
) -> Result<web::Json<User>, Error> {
    let user = update("users", &user_uid, user).await?;

    Ok(user)
}

pub async fn delete_user(user_uid: web::Path<String>) -> Result<web::Json<()>, Error> {
    delete("users", &user_uid).await?;

    Ok(web::Json(()))
}
