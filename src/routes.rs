use crate::handlers;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/ingest", web::post().to(handlers::ingest));
}
