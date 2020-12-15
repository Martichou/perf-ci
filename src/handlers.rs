use crate::data::models::*;
use crate::data::models_http::*;
use crate::data::schema::bench_stat_values::dsl::bench_stat_values;
use crate::data::schema::bench_stats::dsl::*;
use crate::data::schema::filterable_os::dsl::filterable_os;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use diesel::dsl::{exists, insert_into, select};
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
    let data_bench = NewBenchStats {
        branch: &item.branch,
        commit_hash: &item.commit_hash,
        os: &item.os,
        created_at: mcreated_at,
    };
    // Detect if the commit_hash and os already exist
    let does_exists: bool = select(exists(
        bench_stats.filter(commit_hash.eq(&item.commit_hash).and(os.eq(&item.os))),
    ))
    .get_result(&conn)?;
    // If the item already exist, return early
    if does_exists {
        return Ok(());
    }
    // Insert the filterable os value
    insert_into(filterable_os)
        .values(&FilterableOs {
            os: item.os.to_owned(),
        })
        .execute(&conn)?;
    // Insert data
    let inserted_row: BenchStats = insert_into(bench_stats)
        .values(&data_bench)
        .get_result(&conn)?;
    // Construct NewBenchStatsValues
    let mut new_data: Vec<NewBenchStatsValues> = Vec::with_capacity(item.datas.len());
    for data in &item.datas {
        new_data.push(NewBenchStatsValues {
            label: &data.bench,
            mean: data.mean,
            median: data.median,
            slope: data.slope,
            bsid: inserted_row.bsid,
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
        Err(_) => Ok(HttpResponse::BadRequest().await?),
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
