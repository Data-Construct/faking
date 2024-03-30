use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = location_direction_abbr)]
pub fn direction_abbr() -> String {
	EN_DIRECTION_ABBR[seeder::gen_range(0..EN_DIRECTION_ABBR_LEN)].to_string()
}

static EN_DIRECTION_ABBR: [&'static str; 8] = [
    "N",
    "E",
    "S",
    "W",
    "NE",
    "NW",
    "SE",
    "SW",
];
static EN_DIRECTION_ABBR_LEN: usize = EN_DIRECTION_ABBR.len();