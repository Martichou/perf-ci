use crate::data::models::*;
use crate::data::models_http::*;
use crate::data::schema::bench_stat_values::dsl::*;
use crate::data::schema::bench_stats::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareInput {
    hash_a: String,
    hash_b: String,
}

pub async fn compare_hash(
    db: web::Data<Pool>,
    item: web::Json<CompareInput>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route POST /compare_hash");
    }
    // make all get taking advantage of web::block to offload the request thread
    // TODO - Validate the hash_a and hash_b to avoid SQL injection
    // TODO - Handle error too
    let data = web::block(move || get_compare(item, db.get()?)).await;
    match data {
        // Return the json data
        Ok(val) => Ok(HttpResponse::Ok().json(val)),
        // Return a 400
        Err(_) => Ok(HttpResponse::BadRequest().await?)
    }
}

fn get_compare(
    item: web::Json<CompareInput>,
    conn: ConnType,
) -> Result<HttpRawCompareData, AppError> {
    let a: BenchStats = bench_stats
        .filter(crate::data::schema::bench_stats::dsl::commit_hash.eq(&item.hash_a))
        .first::<BenchStats>(&conn)?;
    let b: BenchStats = bench_stats
        .filter(crate::data::schema::bench_stats::dsl::commit_hash.eq(&item.hash_b))
        .first::<BenchStats>(&conn)?;

    let datas_a: Vec<BenchStatsValues> = BenchStatsValues::belonging_to(&a).load(&conn)?;
    let datas_b: Vec<BenchStatsValues> = BenchStatsValues::belonging_to(&b).load(&conn)?;

    let mut datas: Vec<HttpCompareData> = Vec::with_capacity(datas_a.len());
    let mut containing: HashMap<String, usize> = HashMap::with_capacity(datas_a.len());
    for (counter, d) in datas_a.into_iter().enumerate() {
        datas.push(HttpCompareData {
            bench: d.label.to_owned(),
            mean_a: (d.mean * 100.0).round() / 100.0,
            mean_b: 0.0,
        });
        containing.insert(d.label, counter);
    }
    for d in datas_b {
        match containing.get(&d.label) {
            Some(position) => {
                datas.get_mut(*position).unwrap().mean_b = (d.mean * 100.0).round() / 100.0;
            }
            None => datas.push(HttpCompareData {
                bench: d.label.to_owned(),
                mean_a: 0.0,
                mean_b: (d.mean * 100.0).round() / 100.0,
            }),
        }
    }

    Ok(HttpRawCompareData {
        branch_a: a.branch,
        branch_b: b.branch,
        datas,
    })
}
