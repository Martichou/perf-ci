use crate::handlers;
use crate::views;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(views::index))
        .route("/ingest", web::post().to(handlers::ingest));
}
