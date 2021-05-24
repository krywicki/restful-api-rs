use std::convert::TryFrom;

use actix_web::{ HttpRequest, Responder, web };
use wither::{ Model, mongodb::{ Database, options::FindOptions }};
use futures::StreamExt;

use crate::{models::User, schemas::{self, Page, UserResponse}};

pub async fn get_users(req: HttpRequest, db: web::Data<Database>) -> impl Responder {

    //== find users
    // let opts = FindOptions::builder().limit(30).build();
    // let cursor = User::find(&**db, None, opts).await.unwrap();
    // let users: Vec<User> = cursor.map(|u| u.unwrap()).collect().await;

    //== find users (simple)
    let opts = FindOptions::builder().limit(30).build();
    let cursor = db.collection(User::COLLECTION_NAME).find(None, opts).await.unwrap();
    //let users: Vec<Document> = cursor.map(|u| u.unwrap()).collect().await;

    let users: Vec<UserResponse> = cursor
        .map(|u| UserResponse::try_from(u.unwrap()).unwrap())
        .collect().await;

    web::Json(Page::from(users))
}
