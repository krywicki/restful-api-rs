use std::{borrow::Cow, collections::HashMap, fmt};

use serde::Serialize;
use serde_json::{Map, json, value};

use validator::{ValidationError, ValidationErrors};
use wither::{ bson::{ document::ValueAccessError }, mongodb };
use actix_web::{HttpResponse, error::{QueryPayloadError, ResponseError}, http::{self, StatusCode, header}};
use derive_more::{Error};

#[derive(Debug, Error)]
pub struct ErrorResponse {
    pub code:StatusCode,
    pub message:String,
    pub detail: Option<serde_json::Value>
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = json!({
            "message":self.message,
            "detail":self.detail
        }).to_string();

        write!(f, "{}", json)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        let json = json!({
            "message": self.message,
            "detail": self.detail
        });

        HttpResponse::build(self.status_code()).json(json)
    }
}

///
/// Convert ValueAccessError to ErrorResponse
///
impl From<ValueAccessError> for ErrorResponse {
    fn from(error: ValueAccessError) -> Self {
        ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error".into(),
            detail: Some(error.to_string().into())
        }
    }
}

///
/// Convert mongodb error to ErrorResponse
///
impl From<wither::mongodb::error::Error> for ErrorResponse {
    fn from(error: wither::mongodb::error::Error) -> Self {
        ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error".into(),
            detail: Some(format!("{}", error).into())
        }
    }
}

//
// Convert Validation Errors into ErrorResponse
//
impl From<ValidationError> for ErrorResponse {
    fn from(error: ValidationError) -> Self {
        ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: error.message.unwrap_or(Cow::from("Validation Error")).into(),
            detail: Some(value::to_value(error.params).unwrap())
        }
    }
}

impl From<ValidationErrors> for ErrorResponse {
    fn from(error: ValidationErrors) -> Self {
        ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: error.to_string(),
            detail: None
        }
    }
}
