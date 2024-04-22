use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to our Rust API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
           
