use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = location_city_prefix)]
pub fn city_prefix() -> String {
	EN_CITY_PREFIX[seeder::gen_range(0..EN_CITY_PREFIX_LEN)].to_string()
}

static EN_CITY_PREFIX: [&'static str; 8] = [
    "North",
    "East",
    "West",
    "South",
    "New",
    "Lake",
    "Port",
    "Fort",
];
static EN_CITY_PREFIX_LEN: usize = EN_CITY_PREFIX.len();
