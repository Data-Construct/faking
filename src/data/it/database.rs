use crate::{data::datetime::unix::unix_ts_gen, utils::seeder::{self, gen_range}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mongodb_objectid() -> String {
    // 6359388c80616b1fc6d7ec71 parts:
    // 6359388c -- epoc timestamp   1666791564      4 bytes
    // 80616b1fc6 -- random value generated once per process.  551390224326     5 bytes
    // d7ec71 -- incrementing counter which starts from random value (ID)  14150769         3 bytes

    let min: u64 = 4294967296;
    let max: u64 = 1099511627776;
    let rand = seeder::gen_range(min..max);

    let min = 65536;
    let max = 16777216;
    let counter = seeder::gen_range(min..max);

    let ts = unix_ts_gen();

    format!("{:02x}{:02x}{:02x}", ts.1, rand, counter)
}