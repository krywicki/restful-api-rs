use std::convert::TryFrom;
use std::convert::From;

use actix_web::ResponseError;
use actix_web::{ HttpRequest, Responder, web, http};
use serde_json::json;
use validator::Validate;
use wither::bson::doc;
use wither::{ Model, mongodb::{ Database, options::FindOptions }};
use futures::{StreamExt, TryStreamExt};

use crate::web::Query;
use crate::{ models::User, error::ErrorResponse};
use crate::schemas::{ Page, users::{UserResponse, GetUsersParams}};


pub async fn get_users(_: HttpRequest, params: Query<GetUsersParams>, db: web::Data<Database>) -> Result<impl Responder, ErrorResponse> {

    //== find users (simple)
    let opts = FindOptions::builder().limit(params.limit).build();
    let cursor = db.collection(User::COLLECTION_NAME).find(params.make_filter(), opts).await?;
    let users: Vec<UserResponse> = cursor
        .map(|u| {
            UserResponse::try_from(u.unwrap_or_default())
        })
        .try_collect().await?;

    Ok(web::Json(Page::from(users)))
}
