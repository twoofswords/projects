use actix_web::{web, App, HttpServer, Responder};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn hello() -> impl Responder {
    format!("Hello fellow Rustacean!")
}
