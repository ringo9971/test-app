use std::time::Instant;

use actix::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::auth::{check_permission, Token};
use crate::config::Config;
use crate::error::Error;
use crate::firestore;
use crate::models::chat::{Chat, Message};
use crate::websocket::{server, session};

pub async fn game_route(
    req: HttpRequest,
    token: Token,
    game_id: web::Path<String>,
    stream: web::Payload,
    srv: web::Data<Addr<server::GameServer>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    check_permission(&config.auth, &token)?;

    ws::start(
        session::WsGameSession {
            id: 0,
            hb: Instant::now(),
            room: game_id.to_string(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
    .map_err(Error::ResponseError)
}

pub async fn write_message(
    token: Token,
    room_id: web::Path<String>,
    message: web::Json<Message>,
    config: web::Data<Config>,
) -> Result<web::Json<Chat>, Error> {
    check_permission(&config.auth, &token)?;

    let chat = firestore::write_message(
        &config.firestore,
        &room_id.into_inner(),
        message.into_inner(),
    )
    .await?;

    Ok(web::Json(chat))
}

pub async fn get_chat(
    token: Token,
    room_id: web::Path<String>,
    config: web::Data<Config>,
) -> Result<web::Json<Chat>, Error> {
    check_permission(&config.auth, &token)?;

    let chat = firestore::get_chat(&config.firestore, &room_id.into_inner()).await?;

    Ok(web::Json(chat))
}
