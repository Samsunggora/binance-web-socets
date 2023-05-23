use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyData {
    pub E: u64,
    pub e: String,
    pub k: MyNestedData,
    pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyNestedData {
    pub B: String,
    pub L: u64,
    pub Q: String,
    pub T: u64,
    pub V: String,
    pub c: String,
    pub f: u64,
    pub h: String,
    pub i: String,
    pub l: String,
    pub n: u64,
    pub o: String,
    pub q: String,
    pub s: String,
    pub t: u64,
    pub v: String,
    pub x: bool,
}

#[derive(Serialize)]
pub struct StreamData {
    pub stream: String,
    pub data: EndData,
}

#[derive(Serialize)]
pub struct EndData {
    pub timestamp: u64,
    pub open_price: f64,
    pub close_price: f64,
    pub high_price: f64,
    pub low_price: f64,
}
