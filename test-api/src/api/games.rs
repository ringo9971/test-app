use std::time::Instant;

use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

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
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
