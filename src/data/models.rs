use crate::data::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

// Query struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(BenchStats, foreign_key = "commit_hash")]
#[table_name = "bench_stat_values"]
pub struct BenchStatsValues {
    pub id: i32,
    pub label: String,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
    pub commit_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "bench_stats"]
#[primary_key(commit_hash)]
pub struct BenchStats {
    pub branch: String,
    pub commit_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable struct
#[derive(Insertable)]
#[table_name = "bench_stat_values"]
pub struct NewBenchStatsValues<'a> {
    pub label: &'a str,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
    pub commit_hash: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
