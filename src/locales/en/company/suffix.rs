use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = company_suffix)]
pub fn suffix() -> String {
	EN_SUFFIX[seeder::gen_range(0..EN_SUFFIX_LEN)].to_string()
}

static EN_SUFFIX: [&'static str; 4] = ["Inc", "and Sons", "LLC", "Group"];
static EN_SUFFIX_LEN: usize = EN_SUFFIX.len();