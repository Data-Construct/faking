use crate::utils::seeder;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(js_name = location_city_suffix)]
pub fn city_suffix() -> String {
	EN_CITY_SUFFIX[seeder::gen_range(0..EN_CITY_SUFFIX_LEN)].to_string()
}

static EN_CITY_SUFFIX: [&'static str; 23] = [
    "town",
    "ton",
    "land",
    "ville",
    "berg",
    "burgh",
    "boro",
    "borough",
    "bury",
    "view",
    "port",
    "mouth",
    "stad",
    "stead",
    "furt",
    "chester",
    "cester",
    "fort",
    "field",
    "haven",
    "side",
    "shire",
    "worth",
];
static EN_CITY_SUFFIX_LEN: usize = EN_CITY_SUFFIX.len();
