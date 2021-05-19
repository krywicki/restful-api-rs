extern crate api;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use api::endpoints;

async fn hello() -> impl Responder {
    "hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(hello))
            .route("/users", web::get().to(endpoints::users::get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
