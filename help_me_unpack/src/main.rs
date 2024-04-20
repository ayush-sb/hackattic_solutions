/// Original problem statement - https://hackattic.com/challenges/help_me_unpack/
mod utils;

use dotenv::{dotenv, var};
use std::collections::HashMap;
use utils::{
    base64::{base64_to_index, index_to_octet},
    conversion::{bytes_to_int, bytes_to_short, bytes_to_unsigned_int},
    response::ProblemResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load problem and submission URL from .env file in crate root
    // .env file contains
    //      * PROBLEM_URL
    //      * SOLUTION_URL
    dotenv().ok();
    let problem_url = var("PROBLEM_URL").unwrap();
    let solution_url = var("SOLUTION_URL").unwrap();

    // Get problem data, encoded as a packet of bytes
    // The pack contains, always in the following order:
    //      * a regular int (signed), to start off
    //      * an unsigned int
    //      * a short (signed) to make things interesting
    //      * a float because floating point is important
    //      * a double as well
    //      * another double but this time in big endian (network byte order)

    let resp = reqwest::get(problem_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("problem data = {}\n", resp.get("bytes").unwrap());

    // Unwrap data as byte vector
    let data: Vec<char> = resp.get("bytes").unwrap().chars().into_iter().collect();

    // Convert to u8 representing index in base64 alphabet
    let data_index: Vec<u8> = base64_to_index(&data);
    println!("base64 indices = {:?}\n", data_index);

    // Convert to bytes
    let data_bytes: Vec<u8> = index_to_octet(&data_index);
    println!("base64 to binary =");
    for i in &data_bytes {
        println!("{:#010b}", i);
    }

    // Convert to final data
    // Word size is 4 bytes, meaning we skip two bytes after short
    // Bytes to float and double is yet to be implemented
    let res1: i32 = bytes_to_int(&data_bytes[0..4]);
    let res2: u32 = bytes_to_unsigned_int(&data_bytes[4..8]);
    let res3: i16 = bytes_to_short(&data_bytes[8..10]);

    let float_slice = &data_bytes[12..16];
    let res4: f64 = f32::from_le_bytes(float_slice.try_into().unwrap()) as f64;

    let le_double_slice = &data_bytes[16..24];
    let res5: f64 = f64::from_le_bytes(le_double_slice.try_into().unwrap());

    let be_double_slice = &data_bytes[24..32];
    let res6: f64 = f64::from_be_bytes(be_double_slice.try_into().unwrap());

    let problem_response = ProblemResponse {
        int: res1,
        uint: res2,
        short: res3,
        float: res4,
        double: res5,
        big_endian_double: res6,
    };

    println!("final values = {:?}", problem_response);

    let json_body = serde_json::to_string(&problem_response)?;

    let client = reqwest::Client::new();
    let resp = client
        .post(solution_url)
        .body(json_body)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("\nresponse = {:?}", resp);

    Ok(())
}
