use std::net::SocketAddr;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use test_api::{
    api::{create_user::create_user, get_users::get_users},
    error::Error,
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    HttpServer::new(|| {
        App::new()
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
