use actix_web::{web, App, HttpServer};
use controllers::websocket::{connected, count};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod controllers;
mod pojo;   

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .expect("tracing setting default failed");
    HttpServer::new(move || {
        App::new()
            .route("/ws", web::get().to(connected))
            .service(count)
    })
    .bind(("0.0.0.0", 10090))?
    .run()
    .await
}
