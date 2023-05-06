pub use crate::pojo::Session;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use tracing::info;

pub async fn connected(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    for (header_name, header_value) in req.headers().iter() {
        info!(
            "name, value: {}, {}",
            header_name,
            header_value.to_str().unwrap()
        );
    }
    let res = ws::start(Session::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/")]
pub async fn count() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}
