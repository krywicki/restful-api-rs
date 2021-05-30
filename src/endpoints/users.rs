use std::convert::TryFrom;
use std::convert::From;

use actix_web::ResponseError;
use actix_web::{ HttpRequest, Responder, web, http};
use serde_json::json;
use validator::Validate;
use wither::{ Model, mongodb::{ Database, options::FindOptions }};
use futures::{StreamExt, TryStreamExt};

use crate::{ models::User, error::ErrorResponse};
use crate::schemas::{ Page, users::{UserResponse, GetUsersParams}};

pub async fn get_users(_: HttpRequest, params: web::Query<GetUsersParams>, db: web::Data<Database>) -> Result<impl Responder, ErrorResponse> {

    // return Err(ErrorResponse {
    //     code: http::StatusCode::BAD_REQUEST,
    //     message: "how now".into(),
    //     detail: None
    // });
    // params.validate()?;
    // params.pagination.validate()?;


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
