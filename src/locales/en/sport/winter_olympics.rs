use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_winter_olympics)]
pub fn winter_olympics_sport() -> String {
	EN_WINTER_OLYMPICS[seeder::gen_range(0..EN_WINTER_OLYMPICS_LEN)].to_string()
}

static EN_WINTER_OLYMPICS: [&'static str; 16] = [
    "Alpine skiing",
    "Biathlon",
    "Bobsleigh",
    "Cross-country skiing",
    "Curling",
    "Figure skating",
    "Freestyle skiing",
    "Ice hockey",
    "Luge",
    "Nordic combined",
    "Short track speed skating",
    "Skeleton",
    "Ski jumping",
    "Ski mountaineering",
    "Snowboard",
    "Speed skating",
];
static EN_WINTER_OLYMPICS_LEN: usize = EN_WINTER_OLYMPICS.len();