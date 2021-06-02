use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::{fmt, ops};

use serde::{Deserialize, de};
use actix_web::dev::Payload;
use actix_web::error::QueryPayloadError;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use futures::future::{err, ok, Ready};
use validator::{Validate, ValidationError, Validator};

use crate::error::ErrorResponse;

pub struct Query<T: Validate>(pub T);

impl<T> Deref for Query<T>
    where T: Validate
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for Query<T>
    where T: de::DeserializeOwned + Validate,
{
    type Error = ErrorResponse;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &actix_web::web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        serde_urlencoded::from_str::<T>(req.query_string())
            .map_err(ErrorResponse::from)
            .and_then(|q| {
                q.validate().map(move |_| q).map_err(ErrorResponse::from)
            })
            .map_err(ErrorResponse::from)
            .map(|value| ok(Query(value)))
            .unwrap_or_else(|e| {
                err(e)
            })
    }
}

impl From<serde_urlencoded::de::Error> for ErrorResponse {
    fn from(error: serde_urlencoded::de::Error) -> Self {

        ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: "Query Parameter Error".into(),
            detail: Some(error.to_string().into())
        }
    }
}
