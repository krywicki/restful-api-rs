use std::convert::TryFrom;
use std::convert::From;

use actix_web::{ HttpRequest, Responder, web };
use wither::{ Model, mongodb::{ Database, options::FindOptions }};
use futures::{StreamExt, TryStreamExt};

use crate::{ Result, models::User, schemas::{ Page, UserResponse }};

pub async fn get_users(_: HttpRequest, db: web::Data<Database>) -> Result<impl Responder> {

    //== find users (simple)
    let opts = FindOptions::builder().limit(30).build();
    let cursor = db.collection(User::COLLECTION_NAME).find(None, opts).await?;
    let users: Vec<UserResponse> = cursor
        .map(|u| {
            UserResponse::try_from(u.unwrap_or_default())
        })
        .try_collect().await?;

    Ok(web::Json(Page::from(users)))
}
