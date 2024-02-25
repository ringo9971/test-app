use crate::error::Error;
use crate::firestore::delete;
use actix_web::web;

pub async fn delete_user(user_uid: web::Path<String>) -> Result<web::Json<()>, Error> {
    delete("users", &user_uid).await?;

    Ok(web::Json(()))
}
