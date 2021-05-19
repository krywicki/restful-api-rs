use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use crate::schemas;

pub async fn get_users(req: HttpRequest) -> impl Responder {
    web::Json(schemas::UserResponse {
        firstName: String::from("joe"),
        lastName: String::from("krywicki"),
        email: String::from("joe.krywicki@gmail.com")
    })
}