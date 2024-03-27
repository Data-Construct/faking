use chrono::{NaiveDateTime};
use wasm_bindgen::prelude::*;

use super::date_naive::gen_date_naive;
use super::time_naive::{gen_time_naive, gen_time_naive_micro, gen_time_naive_milli, gen_time_naive_nano};

const DATETIME_FORMAT: &str = "%Y-%m-%d";
const DATETIME_FORMAT_MILLI: &str = "%Y-%m-%d %H:%M:%S%.3f";
const DATETIME_FORMAT_MICRO: &str = "%Y-%m-%d %H:%M:%S%.6f";
const DATETIME_FORMAT_NANO: &str = "%Y-%m-%d %H:%M:%S%.9f";

#[wasm_bindgen]
pub fn datetime_naive() -> String {
  let ndt = gen_datetime_naive();
  if ndt.is_none() {
    return "".to_string();
  }
  ndt.unwrap().format(DATETIME_FORMAT).to_string()
}

#[wasm_bindgen]
pub fn datetime_naive_milli() -> String {
  let ndt = gen_datetime_naive_milli();
  if ndt.is_none() {
    return "".to_string();
  }
  ndt.unwrap().format(DATETIME_FORMAT_MILLI).to_string()
}

#[wasm_bindgen]
pub fn datetime_naive_micro() -> String {
  let ndt = gen_datetime_naive_micro();
  if ndt.is_none() {
    return "".to_string();
  }
  ndt.unwrap().format(DATETIME_FORMAT_MICRO).to_string()
}

#[wasm_bindgen]
pub fn datetime_naive_nano() -> String {
  let ndt = gen_datetime_naive_nano();
  if ndt.is_none() {
    return "".to_string();
  }
  ndt.unwrap().format(DATETIME_FORMAT_NANO).to_string()
}

pub fn gen_datetime_naive() -> Option<NaiveDateTime> {
  let date = gen_date_naive();
  if date.is_none() {
    return None;
  }

  let time = gen_time_naive();
  if time.is_none() {
    return None;
  }

  Some(NaiveDateTime::new(date.unwrap(), time.unwrap()))
}

pub fn gen_datetime_naive_milli() -> Option<NaiveDateTime> {
  let date = gen_date_naive();
  if date.is_none() {
    return None;
  }

  let time = gen_time_naive_milli();
  if time.is_none() {
    return None;
  }
  
  Some(NaiveDateTime::new(date.unwrap(), time.unwrap()))
}

pub fn gen_datetime_naive_micro() -> Option<NaiveDateTime> {
  let date = gen_date_naive();
  if date.is_none() {
    return None;
  }

  let time = gen_time_naive_micro();
  if time.is_none() {
    return None;
  }

  Some(NaiveDateTime::new(date.unwrap(), time.unwrap()))
}

pub fn gen_datetime_naive_nano() -> Option<NaiveDateTime> {
  let date = gen_date_naive();
  if date.is_none() {
    return None;
  }

  let time = gen_time_naive_nano();
  if time.is_none() {
    return None;
  }

  Some(NaiveDateTime::new(date.unwrap(), time.unwrap()))
}
