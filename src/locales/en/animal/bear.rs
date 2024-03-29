use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_bear)]
pub fn bear() -> String {
	EN_BEAR[seeder::gen_range(0..EN_BEAR_LEN)].to_string()
}

static EN_BEAR: [&'static str; 8] = [
  "Giant panda",
  "Spectacled bear",
  "Sun bear",
  "Sloth bear",
  "American black bear",
  "Asian black bear",
  "Brown bear",
  "Polar bear",
];
static EN_BEAR_LEN: usize = EN_BEAR.len();