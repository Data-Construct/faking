use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_ancient_olympics)]
pub fn ancient_olympics_sport() -> String {
	EN_ANCIENT_OLYMPICS[seeder::gen_range(0..EN_ANCIENT_OLYMPICS_LEN)].to_string()
}

static EN_ANCIENT_OLYMPICS: [&'static str; 9] = [
    "Boxing",
    "Chariot racing",
    "Discus",
    "Horse racing",
    "Long jump",
    "Pankration",
    "Pentathlon",
    "Running",
    "Wrestling",
];
static EN_ANCIENT_OLYMPICS_LEN: usize = EN_ANCIENT_OLYMPICS.len();