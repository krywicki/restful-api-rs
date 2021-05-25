pub mod schemas;
pub mod endpoints;
pub mod middleware;
pub mod utils;
pub mod models;

mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
