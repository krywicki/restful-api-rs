extern crate api;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use r2d2::Pool;

use api::endpoints;
use api::db_pool::WitherConnectionManger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let manager = WitherConnectionManger::from_uri("");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move ||{
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(endpoints::users::get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
