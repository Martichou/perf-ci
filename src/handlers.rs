use crate::data::models::*;
use crate::data::models_http::HttpRawData;
use crate::data::schema::bench_stat_values::dsl::*;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub async fn ingest(
    db: web::Data<Pool>,
    item: web::Json<HttpRawData>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route POST /ingest : {:?}", item);
    }
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || insert_all_block(item, db.get()?)).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

fn insert_all_block(item: web::Json<HttpRawData>, conn: ConnType) -> Result<(), AppError> {
    // Received time is so the created time
    let mcreated_at = Utc::now().naive_local();
    // Construct BenchStats
    let data_bench = BenchStats {
        branch: item.branch.to_owned(),
        commit_hash: item.commit_hash.to_owned(),
        created_at: mcreated_at,
    };
    // Insert or update if conflict
    let inserted = insert_into(bench_stats)
        .values(&data_bench)
        // damnit, conflict as there are multiple commit_hash in the file (dsls)
        .on_conflict(crate::data::schema::bench_stats::commit_hash)
        .do_nothing()
        .execute(&conn)?;
    // If we don't insert anything, don't store it another time
    // TODO - Might be usefull to instead of do_nothing, change the branch name
    if inserted == 0 {
        return Ok(());
    }
    // Construct NewBenchStatsValues
    let mut new_data: Vec<NewBenchStatsValues> = Vec::with_capacity(item.datas.len());
    for data in &item.datas {
        new_data.push(NewBenchStatsValues {
            label: &data.bench,
            mean: data.mean,
            median: data.median,
            slope: data.slope,
            commit_hash: &item.commit_hash,
            created_at: mcreated_at,
        })
    }
    // Insert the disks
    insert_into(bench_stat_values)
        .values(&new_data)
        .execute(&conn)?;
    // Return Ok(()) as everything went fine
    Ok(())
}
