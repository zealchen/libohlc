use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
#[serde()]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct TickData {
    e: String,
    u: u64,
    pub s: String,
    pub b: String,
    B: String,
    pub a: String,
    A: String,
    pub T: u64,
    E: u64,

    pub b_f: Option<f64>,
    pub a_f: Option<f64>
}

#[derive(Serialize, Deserialize)]
#[serde()]
#[allow(dead_code)]
pub struct OHLCData {
    pub symbol: String,
    pub timestamp: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String
}

#[derive(Deserialize)]
#[serde()]
#[allow(dead_code)]
pub struct OHLCWindow {
    pub open: f64,
    pub high: f64,
    pub low: f64,

    pub begin_index: usize  // the window's begin index of the vector
}

#[allow(non_snake_case)]
impl TickData {
    pub fn new(e: String, u: u64, s: String, b: String, B: String, a: String, A: String, T: u64, E: u64,
            b_f: Option<f64>, a_f: Option<f64>) -> Self {
        TickData{e, u, s, b, B, a, A, T, E, b_f, a_f}
    }
}