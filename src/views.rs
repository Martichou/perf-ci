use crate::data::models::*;
use crate::data::models_http::*;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use askama::Template;
use diesel::prelude::*;
use std::collections::HashMap;

fn get_compare(
    hash_a: String,
    hash_b: String,
    conn: ConnType,
) -> Result<HttpRawCompareData, AppError> {
    let a: BenchStats = bench_stats
        .filter(commit_hash.eq(hash_a))
        .first::<BenchStats>(&conn)?;
    let b: BenchStats = bench_stats
        .filter(commit_hash.eq(hash_b))
        .first::<BenchStats>(&conn)?;

    let datas_a: Vec<BenchStatsValues> = BenchStatsValues::belonging_to(&a).load(&conn)?;
    let datas_b: Vec<BenchStatsValues> = BenchStatsValues::belonging_to(&b).load(&conn)?;
    let mut datas_b_iter = datas_b.into_iter();

    // TODO - Iterate over the one who have the most entry (data_a or data_b)
    // TODO - Need to rework this to make it faster & more reliable. I KNOW IT'S MESSY
    let mut datas: Vec<HttpCompareData> = Vec::with_capacity(datas_a.len());
    for d in datas_a {
        let datas_b_val = datas_b_iter.find(|x| x.label == d.label);
        if datas_b_val.is_some() {
            let datas_b_val = datas_b_val.unwrap();
            datas.push(HttpCompareData {
                bench: d.label,
                mean_a: (d.mean * 100.0).round() / 100.0,
                mean_b: (datas_b_val.mean * 100.0).round() / 100.0,
            })
        }
    }

    Ok(HttpRawCompareData {
        branch_a: a.branch,
        branch_b: b.branch,
        commit_hash_a: a.commit_hash,
        commit_hash_b: b.commit_hash,
        datas,
    })
}

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
    datas: Vec<HttpCompareData>,
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
        let query_a_val = query_a.unwrap().to_owned();
        let query_b_val = query_b.unwrap().to_owned();
        // TODO - Check for integrity of query_a_val and query_b_val
        // Skip this part and pass the query_a and query_b in the compare.html
        // and then in the compare.html make a async ajax call to a endpoint which will construct the CompareData
        let datas = web::block(move || get_compare(query_a_val, query_b_val, db.get()?)).await?;
        // Return the response
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Compare {
                commit_a: datas.commit_hash_a,
                commit_b: datas.commit_hash_b,
                datas: datas.datas,
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
