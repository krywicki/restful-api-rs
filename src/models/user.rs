use serde::{ Deserialize, Serialize };
use wither::prelude::Model;
use wither::bson::{ oid::ObjectId };

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(collection_name="users")]
pub struct User {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,

    pub email: String,

    #[serde(rename="firstName")]
    pub first_name: String,

    #[serde(rename="lastName")]
    pub last_name: String,
}
