use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::websocket::server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug)]
pub struct WsGameSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<server::GameServer>,
}

impl WsGameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(server::Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsGameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                room: self.room.to_owned(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for WsGameSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let Ok(msg) = msg else {
            ctx.stop();
            return;
        };

        println!("websocket message: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => self.addr.do_send(server::ClientMessage {
                id: self.id,
                msg: text.to_string(),
                room: self.room.clone(),
            }),
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
