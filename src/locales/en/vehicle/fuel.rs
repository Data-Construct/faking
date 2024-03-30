use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = vehicle_fuel)]
pub fn fuel() -> String {
	EN_FURL[seeder::gen_range(0..EN_FURL_LEN)].to_string()
}

static EN_FURL: [&'static str; 4] = ["Diesel", "Electric", "Gasoline", "Hybrid"];
static EN_FURL_LEN: usize = EN_FURL.len();