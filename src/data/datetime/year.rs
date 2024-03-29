use wasm_bindgen::prelude::*;
use super::internals;

#[wasm_bindgen]
pub fn year() -> String {
  internals::gen_year().to_string()
}
