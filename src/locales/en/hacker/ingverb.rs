use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_ingverb)]
pub fn ingverb() -> String {
	EN_INGVERB[seeder::gen_range(0..EN_INGVERB_LEN)].to_string()
}

static EN_INGVERB: [&'static str; 16] = [
    "backing up",
    "bypassing",
    "hacking",
    "overriding",
    "compressing",
    "copying",
    "navigating",
    "indexing",
    "connecting",
    "generating",
    "quantifying",
    "calculating",
    "synthesizing",
    "transmitting",
    "programming",
    "parsing",
];
static EN_INGVERB_LEN: usize = EN_INGVERB.len();