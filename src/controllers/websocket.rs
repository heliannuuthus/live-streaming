use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, http:: {
    header:: {
        AUTHORIZATION,
        COOKIE
    }
}};
use actix_web_actors::ws;

struct LsWs {
}

impl LsWs {
    fn new() -> LsWs {
        LsWs {
        }
    }
}

impl Actor for LsWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LsWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("text: {:?}", text);
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

pub async fn connected(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(LsWs::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/")]
pub async fn count() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}
