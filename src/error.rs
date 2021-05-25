use std::fmt;

use serde::Serialize;
use wither::{ bson::{ document::ValueAccessError }, mongodb };
use actix_web::{ error::ResponseError, http, HttpResponse };

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: u16,
    pub error: String,
    pub message: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.code, self.error, self.message)
    }
}

impl From<ValueAccessError> for Error {
    fn from(err: ValueAccessError) -> Error {
        Error {
            code: 500,
            error: "Internal Server Error".into(),
            message: err.to_string()
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Error {
            code: 500,
            error: "Internal Server Error".into(),
            message: error.to_string()
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::from_u16(self.code).unwrap()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(&self)
    }
}
