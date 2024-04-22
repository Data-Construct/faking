use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_winter_paralympics)]
pub fn winter_paralympics_sport() -> String {
	EN_WINTER_PAEALYMPICS[seeder::gen_range(0..EN_WINTER_PAEALYMPICS_LEN)].to_string()
}

static EN_WINTER_PAEALYMPICS: [&'static str; 6] = [
    "Alpine skiing",
    "Biathlon",
    "Cross-country skiing",
    "Para ice hockey",
    "Snowboard",
    "Wheelchair curling",
];
static EN_WINTER_PAEALYMPICS_LEN: usize = EN_WINTER_PAEALYMPICS.len();