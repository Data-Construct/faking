pub mod aircraft_type;
use crate::utils::seeder;
use wasm_bindgen::prelude::*;

/// Returns a random aircraft type.
#[wasm_bindgen]
pub fn aircraft_type() -> String {
	use aircraft_type::{EN_AIRCRAFT_TYPE, EN_AIRCRAFT_TYPE_LEN};
	EN_AIRCRAFT_TYPE[seeder::gen_range(0..EN_AIRCRAFT_TYPE_LEN)].to_string()
}
