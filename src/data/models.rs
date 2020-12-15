use crate::data::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

// Query struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(BenchStats, foreign_key = "bsid")]
#[table_name = "bench_stat_values"]
pub struct BenchStatsValues {
    pub id: i32,
    pub label: String,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
    pub bsid: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "bench_stats"]
#[primary_key(bsid)]
pub struct BenchStats {
    pub bsid: i32,
    pub branch: String,
    pub commit_hash: String,
    pub os: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "filterable_os"]
#[primary_key(os)]
pub struct FilterableOs {
    pub os: String,
}

// Insertable struct
#[derive(Insertable)]
#[table_name = "bench_stat_values"]
pub struct NewBenchStatsValues<'a> {
    pub label: &'a str,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
    pub bsid: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "bench_stats"]
pub struct NewBenchStats<'a> {
    pub branch: &'a str,
    pub commit_hash: &'a str,
    pub os: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
