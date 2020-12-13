use crate::data::models::*;
use crate::data::models_http::*;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub fn get_bench_vec(size: i64, page: i64, conn: ConnType) -> Result<Vec<HttpViewData>, AppError> {
    let bench_data: Vec<BenchStats> = bench_stats.limit(size).offset(page * size).load(&conn)?;

    let mut raw_data: Vec<HttpViewData> = Vec::with_capacity(bench_data.len());
    for bd in bench_data {
        let datas = BenchStatsValues::belonging_to(&bd).first::<BenchStatsValues>(&conn)?;
        raw_data.push(HttpViewData {
            branch: bd.branch,
            commit_hash: bd.commit_hash,
            datas,
        });
    }

    Ok(raw_data)
}

pub async fn index(db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route GET /");
    }

    // Get the last 10 benchmark results
    let data = web::block(move || get_bench_vec(10, 0, db.get()?)).await?;

    Ok(HttpResponse::Ok().json(data))
}
