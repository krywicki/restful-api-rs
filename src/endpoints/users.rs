use actix_web::{App, HttpRequest, Responder, web::{self, Data}};
use wither::Model;

use crate::{ schemas, models::{self, User}, DbPool };

pub async fn get_users(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {

    let conn = pool.get().expect("failed getting db connection from pool");

    let users = User::find(&**conn, None, None).await;


    web::Json(schemas::UserResponse {
        first_name: String::from("joe"),
        last_name: String::from("krywicki"),
        email: String::from("joe.krywicki@gmail.com")
    })
}
