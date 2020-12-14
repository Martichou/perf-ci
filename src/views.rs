use crate::data::models::*;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use askama::Template;
use diesel::prelude::*;
use std::collections::HashMap;

fn get_summary_vec(size: i64, page: i64, conn: ConnType) -> Result<Vec<BenchStats>, AppError> {
    Ok(bench_stats
        .limit(size)
        .offset(page * size)
        .order_by(created_at.desc())
        .load(&conn)?)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    datas: Vec<BenchStats>,
}

#[derive(Template)]
#[template(path = "compare.html")]
struct Compare {
    commit_a: String,
    commit_b: String,
}

pub async fn index(
    db: web::Data<Pool>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route GET /");
    }

    let query_a = query.get("a");
    let query_b = query.get("b");

    if query_a.is_some() && query_b.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Compare {
                commit_a: query_a.unwrap().to_owned(),
                commit_b: query_b.unwrap().to_owned(),
            }
            .render()
            .unwrap(),
        ))
    } else {
        // Get the last 25 benchmark summary
        let data = web::block(move || get_summary_vec(25, 0, db.get()?)).await?;
        // Return the response
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(Index { datas: data }.render().unwrap()))
    }
}
