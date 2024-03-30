use wasm_bindgen::prelude::*;

use crate::utils::seeder;

const URL_BASE: &str = "https://cloudflare-ipfs.com/ipfs/Qmd3W5DuhgHirLHGVixi6V76LhCkZUz6pnFt5AJBiyvHye/avatar/";

#[wasm_bindgen]
pub fn image_cloudflare_avatar() -> String {
  let n: usize = seeder::gen_range(0..=1249);
  concat_string!(URL_BASE, n.to_string(), ".jpg")
}
