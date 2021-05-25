use actix_web_validator::Validate;
use serde::{ Serialize, Deserialize };

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

#[derive(Deserialize)]
struct GetUsersQueryParams {
    limit: usize,

    offset: usize,

    #[serde(rename="orderBy")]
    order_by: String,
}
