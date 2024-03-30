use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_direction)]
pub fn direction() -> String {
	EN_DIRECTION[seeder::gen_range(0..EN_DIRECTION_LEN)].to_string()
}

static EN_DIRECTION: [&'static str; 8] = [
    "North",
    "East",
    "South",
    "West",
    "Northeast",
    "Northwest",
    "Southeast",
    "Southwest",
];
static EN_DIRECTION_LEN: usize = EN_DIRECTION.len();