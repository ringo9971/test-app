use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use test_api::{api::get_users::get_users, error::Error};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    HttpServer::new(|| App::new().route("/users", web::get().to(get_users)))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
