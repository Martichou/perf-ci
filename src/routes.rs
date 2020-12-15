use crate::handlers;
use crate::validator;
use crate::views;

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator::validator);
    cfg.route("/", web::get().to(views::index))
        .route("/compare_hash", web::post().to(handlers::compare_hash))
        .service(
            web::scope("/api")
                .wrap(auth)
                .route("/ingest", web::post().to(handlers::ingest)),
        );
}
