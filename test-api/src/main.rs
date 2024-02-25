use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use test_api::{
    api::users::{create_user, delete_user, get_users, update_user},
    error::Error,
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let allowed_origin = env::var("APP_BASE_URL").expect("failed get env var (APP_BASE_URL)");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{user_uid}", web::put().to(update_user))
            .route("/users/{user_uid}", web::delete().to(delete_user))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
