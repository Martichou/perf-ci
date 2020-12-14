#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod data;
mod errors;
mod handlers;
mod routes;
mod server;
mod validator;
mod views;

use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpPostData {
    pub bench: String,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();
    // Continue the initialization of the actix web server
    // And wait indefinietly for it <3
    server::server().await
}
