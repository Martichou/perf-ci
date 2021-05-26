use crate::data::models::{BenchStats, FilterableOs};
use crate::data::schema::{
    bench_stats::dsl::{bench_stats as dsl_bench, created_at},
    filterable_os::dsl::*,
};
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use askama::Template;
use diesel::*;
use std::collections::HashMap;

fn get_filterable_os(conn: ConnType) -> Result<Vec<FilterableOs>, AppError> {
    Ok(filterable_os.load(&conn)?)
}

fn get_commits(conn: ConnType) -> Result<Vec<BenchStats>, AppError> {
    Ok(dsl_bench
        .limit(20)
        .order_by(created_at.desc())
        .load(&conn)?)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    commits: Vec<BenchStats>,
}

#[derive(Template)]
#[template(path = "compare.html")]
struct Compare {
    commit_a: String,
    commit_b: String,
    data: Vec<FilterableOs>,
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
        // Get the filterable os here
        let data = web::block(move || get_filterable_os(db.get()?)).await?;
        // Return the response
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Compare {
                commit_a: query_a.unwrap().to_owned(),
                commit_b: query_b.unwrap().to_owned(),
                data,
            }
            .render()
            .unwrap(),
        ))
    } else {
        let data = web::block(move || get_commits(db.get()?)).await?;
        // Return the response
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(Index { commits: data }.render().unwrap()))
    }
}
