use serde::{ Deserialize, Serialize };
use wither::prelude::Model;
use wither::bson::{ oid::ObjectId };

#[derive(Debug, Model, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[model(collection_name="users")]
pub struct User {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
