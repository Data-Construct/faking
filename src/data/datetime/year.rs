use wasm_bindgen::prelude::*;
use super::internals;

#[wasm_bindgen]
pub fn year() -> String {
  internals::gen_year_epoch().to_string()
}

#[wasm_bindgen]
pub fn year_unsafe() -> String {
  internals::gen_year_unsafe().to_string()
}
