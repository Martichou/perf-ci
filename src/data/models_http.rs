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
pub struct HttpCompareData {
    pub bench: String,
    pub mean_a: f64,
    pub mean_b: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRawCompareData {
    pub branch_a: String,
    pub branch_b: String,
    pub commit_hash_a: String,
    pub commit_hash_b: String,
    pub datas: Vec<HttpCompareData>,
}
