use std::sync::Arc;
use std::{fmt, ops};

use serde::de;
use actix_web::dev::Payload;
use actix_web::error::QueryPayloadError;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use futures::future::{err, ok, Ready};
use validator::{ Validate, ValidationError };
