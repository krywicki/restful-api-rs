use std::convert::{ From, TryFrom };

use serde::{ Serialize, Deserialize };
use wither::bson;
use validator::{ Validate, ValidationError };

use crate::Error;

pub mod users;


#[derive(Serialize)]
pub struct Page<T>
    where T: Serialize
{
    pub next: String,
    pub count: usize,
    pub items: Vec<T>
}

impl<T> From<Vec<T>> for Page<T>
    where T: Serialize
{
    fn from(items: Vec<T>) -> Self {
        Page {
            next: "".into(),
            count: items.len(),
            items: items
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct PageParams {
    #[validate(range(min=1, max=9999))]
    #[serde(default="default_limit")]
    pub limit: usize,

    #[serde(default="default_offset")]
    pub offset: usize,
}

fn default_limit() -> usize { 200 }
fn default_offset() -> usize { 0 }
