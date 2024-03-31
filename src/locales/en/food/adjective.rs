use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = food_adjective)]
pub fn adjective() -> String {
	EN_ADJECTIVE[seeder::gen_range(0..EN_ADJECTIVE_LEN)].to_string()
}

static EN_ADJECTIVE: [&'static str; 20] = [
    "bitter",
    "creamy",
    "crispy",
    "crunchy",
    "delicious",
    "fluffy",
    "fresh",
    "golden",
    "juicy",
    "moist",
    "rich",
    "salty",
    "savory",
    "smoky",
    "sour",
    "spicy",
    "sweet",
    "tangy",
    "tender",
    "zesty",
];
static EN_ADJECTIVE_LEN: usize = EN_ADJECTIVE.len();