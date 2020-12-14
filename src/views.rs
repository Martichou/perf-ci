use crate::errors::AppError;

use actix_web::{web, HttpResponse};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[derive(Template)]
#[template(path = "compare.html")]
struct Compare {
    commit_a: String,
    commit_b: String,
}

pub async fn index(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route GET /");
    }

    let query_a = query.get("a");
    let query_b = query.get("b");

    if query_a.is_some() && query_b.is_some() {
        // Return the response
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Compare {
                commit_a: query_a.unwrap().to_owned(),
                commit_b: query_b.unwrap().to_owned(),
            }
            .render()
            .unwrap(),
        ))
    } else {
        // Return the response
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(Index{}.render().unwrap()))
    }
}
