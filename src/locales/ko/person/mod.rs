use crate::utils::seeder;
use wasm_bindgen::prelude::*;

pub mod job;

#[wasm_bindgen(js_name = ko__person_job)]
pub fn job() -> String {
    use job::{KO__JOB, KO__JOB_LEN};
	KO__JOB[seeder::gen_range(0..KO__JOB_LEN)].to_string()
}
