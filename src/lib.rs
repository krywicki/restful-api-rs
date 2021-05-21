use r2d2::Pool;

pub mod schemas;
pub mod endpoints;
pub mod middleware;
pub mod utils;
pub mod models;
pub mod db_pool;

pub type DbPool = Pool<db_pool::WitherConnectionManger>;
