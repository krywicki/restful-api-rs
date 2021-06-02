use std::convert::{ From, TryFrom };

use serde::{ Serialize, Deserialize };
use wither::bson;
use validator::Validate;


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

fn default_page_limit() -> i64 { 200 }
fn default_page_offset() -> usize { 0 }
