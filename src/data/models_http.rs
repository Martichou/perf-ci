use crate::data::models::BenchStatsValues;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpOneData {
    pub bench: String,
    pub mean: f64,
    pub median: f64,
    pub slope: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRawData {
    pub branch: String,
    pub commit_hash: String,
    pub datas: Vec<HttpOneData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpViewData {
    pub branch: String,
    pub commit_hash: String,
    pub datas: BenchStatsValues,
}
