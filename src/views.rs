use crate::data::models::BenchStats;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub fn get_bench_vec(size: i64, page: i64, conn: ConnType) -> Result<Vec<BenchStats>, AppError> {
    Ok(bench_stats.limit(size).offset(page * size).load(&conn)?)
}

pub async fn index(db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route GET /");
    }

    let data = web::block(move || get_bench_vec(10, 0, db.get()?)).await?;

    Ok(HttpResponse::Ok().json(data))
}
