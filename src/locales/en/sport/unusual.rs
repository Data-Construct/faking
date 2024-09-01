use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_unusual)]
pub fn unusual_sport() -> String {
	EN_UNUSUAL_SPORT[seeder::gen_range(0..EN_UNUSUAL_SPORT_LEN)].to_string()
}

static EN_UNUSUAL_SPORT: [&'static str; 21] = [
    "Apple Racing",
    "Ban'ei",
    "Bathtubbing",
    "Bed racing",
    "Bossaball",
    "Botaoshi",
    "Beer Can Regatta",
    "Black pudding throwing",
    "Bog snorkelling",
    "Bottle kicking",
    "Camel jumping",
    "Camel wrestling",
    "Chess boxing",
    "Extreme ironing",
    "Flugtag/Birdman",
    "Gurning",
    "Kastenlauf (Beer crate running)",
    "Oil wrestling",
    "Poohsticks",
    "Wife carrying",
    "Zorbing",
];
static EN_UNUSUAL_SPORT_LEN: usize = EN_UNUSUAL_SPORT.len();