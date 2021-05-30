extern crate api;

use std::sync::Arc;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use api::endpoints;
use wither::mongodb;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let uri = "mongodb://admin:admin@localhost:27017/?authSource=admin";
    let client = mongodb::Client::with_uri_str(uri).await.expect("failed connecting to db");
    let db = web::Data::new(client.database("production"));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/users", web::get().to(endpoints::users::get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
