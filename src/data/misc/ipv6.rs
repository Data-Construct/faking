use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ipv6_address() -> String {
  let mut slist: Vec<String> = vec![];
  for _ in 0..8 {
    let v = seeder::gen_range(IP_RANGE_MIN..=IP_RANGE_MAX);
    slist.push(format!("{:x}", v));
  }
  slist.join(":")
}

#[wasm_bindgen]
pub fn ipv6_address_with_cidr() -> String {
  let mut s = ipv6_address();
  s.push_str("/");

  let v = seeder::gen_range(CIDR_MIN..=CIDR_MAX).to_string();
  s.push_str(&v);
  s
}

static IP_RANGE_MIN: u32 = 0;
static IP_RANGE_MAX: u32 = 65535;

static CIDR_MIN: u32 = 1;
static CIDR_MAX: u32 = 127;