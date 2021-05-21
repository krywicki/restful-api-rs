use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    #[serde(alias="firstName")]
    pub first_name: String,

    #[serde(alias="lastName")]
    pub last_name: String,

    pub email: String
}

#[derive(Serialize)]
pub struct Page<T:Sized + Serialize> {
    pub next: String,
    pub count: i64,
    pub items: Vec<T>
}
