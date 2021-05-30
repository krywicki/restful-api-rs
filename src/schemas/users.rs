
use std::convert::{ From, TryFrom };

use validator::{ Validate, ValidationError };
use serde::{ Serialize, Deserialize };
use wither::bson;


use crate::schemas::PageParams;
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

    #[serde(flatten)]
    pub pagination: PageParams,

    #[validate(custom="users_order_by")]
    #[serde(default="default_order_by")]
    pub order_by: String,
}

fn default_order_by() -> String { "lastName".into() }

fn users_order_by(value:&str) -> Result<(), ValidationError> {
    Ok(())
}
