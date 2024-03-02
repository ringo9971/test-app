use actix::Actor;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use std::{
    env,
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc},
};
use test_api::api::games::*;
use test_api::error::Error;
use test_api::websocket::server;

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
            .route("games/{game_id}", web::get().to(get_chat))
            .route("games/{game_id}", web::post().to(write_message))
            .route("games/{game_id}/ws", web::get().to(game_route))
    })
    .bind(addr)?
    .workers(3)
    .run()
    .await?;

    Ok(())
}
