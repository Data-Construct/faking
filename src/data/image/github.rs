use wasm_bindgen::prelude::*;

use crate::utils::seeder;

const URL_BASE: &str = "https://avatars.githubusercontent.com/u/";

#[wasm_bindgen]
pub fn image_github_avatar() -> String {
  let n: usize = seeder::gen_range(0..=100000000);
  concat_string!(URL_BASE, n.to_string())
}
