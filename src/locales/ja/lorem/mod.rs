use crate::utils::seeder;
use wasm_bindgen::prelude::*;

pub mod words;

#[wasm_bindgen(js_name = ja__lorem_ipsum_word)]
pub fn word() -> String {
    use words::{JA__LOREM, JA__LOREM_LEN};
	JA__LOREM[seeder::gen_range(0..JA__LOREM_LEN)].to_string()
}
