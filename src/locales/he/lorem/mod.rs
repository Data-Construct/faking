use crate::utils::seeder;
use wasm_bindgen::prelude::*;
pub mod lorem;
use lorem::{HE__LOREM, HE__LOREM_LEN};

#[wasm_bindgen(js_name = he__lorem_ipsum_word)]
pub fn word() -> String {
	HE__LOREM[seeder::gen_range(0..HE__LOREM_LEN)].to_string()
}
