use crate::utils::seeder;
use chrono::Month;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn month() -> String {
  let m = get_random_month();
  if m.is_none() {
    return "".to_string();
  }

  return m.unwrap().name().to_string();
}

#[wasm_bindgen]
pub fn month_abbr() -> String {
  let m = get_random_month();
  if m.is_none() {
    return "".to_string();
  }

  return m.unwrap().name().chars().take(3).collect::<String>();
}

#[wasm_bindgen]
pub fn month_ordinal() -> String {
  (seeder::gen_range(1..=12)).to_string()
}

#[wasm_bindgen]
pub fn month_ordinal0() -> String {
  seeder::gen_range(0..12).to_string()
}

fn get_random_month() -> Option<Month> {
  let i: u8 = seeder::gen_range(1..=12);
  match Month::try_from(i) {
    Ok(m) => return Some(m),
    Err(e) => {
      println!("{}\n", e);
      return None;
    }
  }
}
