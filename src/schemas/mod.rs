use serde::Serialize;
#[derive(Serialize)]
pub struct UserResponse {
    pub firstName: String, 
    pub lastName: String, 
    pub email: String
}