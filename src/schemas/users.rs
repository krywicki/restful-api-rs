
use std::convert::{ From, TryFrom };

use validator::{ Validate, ValidationError };
use serde::{ Serialize, Deserialize };
use wither::bson;
use wither::bson::bson;
use wither::bson::doc;

use crate::schemas;
use crate::error::ErrorResponse;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl TryFrom<bson::Document> for UserResponse {
    type Error = ErrorResponse;

    fn try_from(doc: bson::Document) -> Result<Self, Self::Error> {
        Ok(UserResponse{
            id: doc.get_object_id("_id")?.to_string(),
            first_name: doc.get_str("firstName")?.into(),
            last_name: doc.get_str("lastName")?.into(),
            email: doc.get_str("email")?.into()
        })
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all="camelCase")]
pub struct GetUsersParams {
    pub order_by: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,

    #[validate(range(min=1, max=9_999))]
    #[serde(default="schemas::default_page_limit")]
    pub limit:i64,

    #[serde(default="schemas::default_page_offset")]
    pub offset:usize,
}

impl GetUsersParams {
    pub fn make_filter(&self) -> Option<bson::Document> {
        let mut filter = bson::Document::new();

        if let Some(ref first_name) = self.first_name {
            filter.insert("firstName", bson::Bson::String(first_name.clone()));
        }

        if let Some(ref last_name) = self.last_name {
            filter.insert("lastName", bson::Bson::String(last_name.clone()));
        }

        Some(filter)
    }
}

fn default_order_by() -> String { "lastName".into() }

fn users_order_by(value:&str) -> Result<(), ValidationError> {
    Ok(())
}
