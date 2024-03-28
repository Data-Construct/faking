use wasm_bindgen::prelude::*;
use chrono::{NaiveDate, TimeDelta, Utc};
use crate::utils::seeder;

use super::internals::{gen_days, gen_month, gen_year, YEAR_MAX, YEAR_MIN};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[wasm_bindgen]
pub fn date_naive() -> String {
  let ndo = gen_date_naive();
  if ndo.is_none() {
    return "".to_string();
  }
  return ndo.unwrap().format(DATE_FORMAT).to_string();
}

#[wasm_bindgen]
pub fn date_naive_between(after: String, before: String) -> String {
  let pda = NaiveDate::parse_from_str(after.as_str(), DATE_FORMAT);
  if pda.is_err() {
    print!("{}\n", pda.err().unwrap());
    return "".to_string();
  }

  let pdb = NaiveDate::parse_from_str(before.as_str(), DATE_FORMAT);
  if pdb.is_err() {
    print!("{}\n", pdb.err().unwrap());
    return "".to_string();
  }

  let ndo = gen_date_naive_between(pda.unwrap(), pdb.unwrap());
  if ndo.is_none() {
    return "".to_string();
  }
  return ndo.unwrap().format(DATE_FORMAT).to_string();
}

#[wasm_bindgen]
pub fn date_naive_before(date: String) -> String {
  match NaiveDate::parse_from_str(date.as_str(), DATE_FORMAT) {
      Ok(nd) => {
        let ndo = gen_date_naive_before(nd);
        if ndo.is_none() {
          return "".to_string();
        }
        return ndo.unwrap().format(DATE_FORMAT).to_string();
      },
      Err(e) => {
        print!("{}\n", e);
        return "".to_string();
      }
  }
}

#[wasm_bindgen]
pub fn date_naive_before_today() -> String {
  let ndo = gen_date_naive_before_today();
  if ndo.is_none() {
    return "".to_string();
  }
  return ndo.unwrap().format(DATE_FORMAT).to_string();
}

#[wasm_bindgen]
pub fn date_naive_after(date: String) -> String {
  match NaiveDate::parse_from_str(date.as_str(), DATE_FORMAT) {
    Ok(nd) => {
      let ndo = gen_date_naive_after(nd);
      if ndo.is_none() {
        return "".to_string();
      }
      return ndo.unwrap().format(DATE_FORMAT).to_string();
    },
    Err(e) => {
      print!("{}\n", e);
      return "".to_string();
    }
}
}

#[wasm_bindgen]
pub fn date_naive_after_today() -> String {
  let ndo = gen_date_naive_after_today();
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

pub fn gen_date_naive_between(min: NaiveDate, max: NaiveDate) -> Option<NaiveDate> {
  let delta = max.signed_duration_since(min);
  let seconds = seeder::gen_range(1..=delta.num_seconds());
  let nd = min + TimeDelta::seconds(seconds);
  Some(nd)
}

pub fn gen_date_naive_before(before: NaiveDate) -> Option<NaiveDate> {
  let min = NaiveDate::from_ymd(YEAR_MIN, 1, 1);
  gen_date_naive_between(min, before)
}

pub fn gen_date_naive_before_today() -> Option<NaiveDate> {
  let now = Utc::now();
  let now_nd = now.naive_utc();
  let min = NaiveDate::from_ymd(YEAR_MIN, 1, 1);
  gen_date_naive_between(min, now_nd.date())
}

pub fn gen_date_naive_after(after: NaiveDate) -> Option<NaiveDate> {
  let max = NaiveDate::from_ymd(YEAR_MAX, 12, 31);
  gen_date_naive_between(after, max)
}

pub fn gen_date_naive_after_today() -> Option<NaiveDate> {
  let now = Utc::now();
  let now_nd = now.naive_utc();
  let max = NaiveDate::from_ymd(YEAR_MAX, 12, 31);
  gen_date_naive_between(now_nd.date(), max)
}

