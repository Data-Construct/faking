use wasm_bindgen::prelude::*;
use chrono::{NaiveTime};
use super::internals::{gen_hms, gen_micro, gen_milli, gen_nano};

const TIME_FORMAT: &str = "%H:%M:%S";
const TIME_FORMAT_MILLI: &str = "%H:%M:%S%.3f";
const TIME_FORMAT_MICRO: &str = "%H:%M:%S%.6f";
const TIME_FORMAT_NANO: &str = "%H:%M:%S%.9f";

#[wasm_bindgen]
pub fn time_naive() -> String {
  let nto = gen_time_naive();
  if nto.is_none() {
    return "".to_string();
  }
  return nto.unwrap().format(TIME_FORMAT);
}

#[wasm_bindgen]
pub fn time_naive_milli() -> String {
  let nto = gen_time_naive_milli();
  if nto.is_none() {
    return "".to_string();
  }
  return nto.unwrap().format(TIME_FORMAT_MILLI);
}

#[wasm_bindgen]
pub fn time_naive_micro() -> String {
  let nto = gen_time_naive_micro();
  if nto.is_none() {
    return "".to_string();
  }
  return nto.unwrap().format(TIME_FORMAT_MICRO);
}

#[wasm_bindgen]
pub fn time_naive_nano() -> String {
  let nto = gen_time_naive_nano();
  if nto.is_none() {
    return "".to_string();
  }
  return nto.unwrap().format(TIME_FORMAT_NANO);
}

pub fn gen_time_naive() -> Option<NaiveTime> {
  let (hour, min, sec) = gen_hms();
  NaiveTime::from_hms_opt(hour, min, sec)
}

pub fn gen_time_naive_milli() -> Option<NaiveTime> {
  let (hour, min, sec) = gen_hms();
  let milli = gen_milli();
  NaiveTime::from_hms_milli_opt(hour, min, sec, milli)
}

pub fn gen_time_naive_micro() -> Option<NaiveTime> {
  let (hour, min, sec) = gen_hms();
  let micro = gen_micro();
  NaiveTime::from_hms_micro_opt(hour, min, sec, micro)
}

pub fn gen_time_naive_nano() -> Option<NaiveTime> {
  let (hour, min, sec) = gen_hms();
  let nano = gen_nano();
  NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
}