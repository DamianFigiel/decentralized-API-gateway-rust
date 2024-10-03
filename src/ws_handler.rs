use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use std::time::{Instant};

pub struct WebSocketSession {
    pub hb: Instant,
}

impl WebSocketSession {
    pub fn new() -> Self {
        WebSocketSession {
            hb: Instant::now(),
        }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection established.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection closed.");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received WebSocket message: {}", text);
                ctx.text(format!("Message received: {}", text));
            }
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }

            _ => ctx.stop(),
        }
    }    
}

pub async fn ws_index(req: HttpRequest, stream: actix_web::web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocketSession::new(), &req, stream);
    resp
}