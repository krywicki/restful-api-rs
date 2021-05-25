use std::convert::{ From, TryFrom };
use std::fmt;

use serde::Serialize;
use wither::{ bson::{self, document::ValueAccessError, Document } };
use actix_web::{ error::ResponseError, dev::HttpResponseBuilder, http, web, HttpResponse };

use crate::Error;


#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,

    pub email: String,

    #[serde(rename="firstName")]
    pub first_name: String,

    #[serde(rename="lastName")]
    pub last_name: String,
}

impl TryFrom<bson::Document> for UserResponse {
    type Error = Error;

    fn try_from(doc: bson::Document) -> Result<Self, Self::Error> {
        Ok(UserResponse{
            id: doc.get_object_id("_id")?.to_string(),
            first_name: doc.get_str("firstName")?.into(),
            last_name: doc.get_str("lastName")?.into(),
            email: doc.get_str("email")?.into()
        })
    }
}

#[derive(Serialize)]
pub struct Page<T>
    where T: Serialize
{
    pub next: String,
    pub count: usize,
    pub items: Vec<T>
}


impl<T> From<Vec<T>> for Page<T>
    where T: Serialize
{
    fn from(items: Vec<T>) -> Self {
        Page {
            next: "".into(),
            count: items.len(),
            items: items
        }
    }
}
