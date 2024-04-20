use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProblemResponse {
    pub int: i32,
    pub uint: u32,
    pub short: i16,
    pub float: f64,
    pub double: f64,
    pub big_endian_double: f64,
}
