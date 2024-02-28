use actix::Actor;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use std::{
    env,
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc},
};
use test_api::{
    api::{
        games::game_route,
        users::{create_user, delete_user, get_users, update_user},
    },
    error::Error,
    websocket::server,
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let app_state = Arc::new(AtomicUsize::new(0));
    let server = server::GameServer::new(app_state.clone()).start();

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
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{user_uid}", web::put().to(update_user))
            .route("/users/{user_uid}", web::delete().to(delete_user))
            .route("/ws", web::get().to(game_route))
    })
    .bind(addr)?
    .workers(3)
    .run()
    .await?;

    Ok(())
}
