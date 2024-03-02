use std::time::Instant;

use actix::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::error::Error;
use crate::firestore;
use crate::models::chat::{Chat, Message};
use crate::websocket::{server, session};

pub async fn game_route(
    req: HttpRequest,
    game_id: web::Path<String>,
    stream: web::Payload,
    srv: web::Data<Addr<server::GameServer>>,
) -> Result<HttpResponse, Error> {
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
    room_id: web::Path<String>,
    message: web::Json<Message>,
) -> Result<web::Json<Chat>, Error> {
    let chat = firestore::write_message(&room_id.into_inner(), message.into_inner()).await?;

    Ok(web::Json(chat))
}

pub async fn get_chat(room_id: web::Path<String>) -> Result<web::Json<Chat>, Error> {
    let chat = firestore::get_chat(&room_id.into_inner()).await?;

    Ok(web::Json(chat))
}
