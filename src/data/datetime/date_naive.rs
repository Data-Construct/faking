use wasm_bindgen::prelude::*;
use chrono::NaiveDate;
use super::internals::{gen_days, gen_month, gen_year, get_days_in_month};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[wasm_bindgen]
pub fn date_naive() -> String {
  let ndo = gen_date_naive();
  if ndo.is_none() {
    return "".to_string();
  }
  return ndo.unwrap().format(DATE_FORMAT).to_string();
}

pub fn gen_date_naive() -> Option<NaiveDate> {
  let year = gen_year();
  let month = gen_month();
  let day = gen_days(year, month);
  NaiveDate::from_ymd_opt(year, month, day)
}