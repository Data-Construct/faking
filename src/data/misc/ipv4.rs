use rand::Rng;
use regex::Regex;
use std::ops::Range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ipv4_address() -> String {
  let mut rng = rand::thread_rng();
  let mut slist: Vec<String> = vec![];
  for _ in 0..4 {
    let v = rng.gen_range(IP_RANGE_MIN..=IP_RANGE_MAX).to_string();
    slist.push(v);
  }
  slist.join(".")
}

#[wasm_bindgen]
pub fn ipv4_address_with_cidr() -> String {
  let mut s = ipv4_address();
  s.push_str("/");

  let mut rng = rand::thread_rng();
  let v = rng.gen_range(CIDR_MIN..=CIDR_MAX).to_string();
  s.push_str(&v);
  s
}

#[wasm_bindgen]
pub fn ipv4_address_public() -> String {
  let ctrl = 0;

  // Using 10,000 as a heuristic to ensure loop does not run forever.
  while ctrl < 10000 {
    let ip = ipv4_address();
    if is_private_network(&ip) || is_reserved_network(&ip) {
      continue;
    }
    return ip;
  }
  return "".to_owned();
}

#[wasm_bindgen]
pub fn ipv4_address_private() -> String {
  let mut rng = rand::thread_rng();
  let ranges = PRIVATE_RANGES[rng.gen_range(0..(*PRIVATE_RANGES_LEN))];
  let mut slist: Vec<String> = vec![];
  for i in ranges {
    if i.start == i.end {
      slist.push(i.start.to_string());
      continue;
    }

    let v = rng.gen_range(i.start..i.end).to_string();
    slist.push(v);
  }
  slist.join(".")
}

fn is_private_network(ip: &String) -> bool {
  for elem in PRIVATE_RANGES_REGEX.iter() {
    if (**elem).is_match(ip.as_str()) {
      return true;
    }
  }
  return false;
}

fn is_reserved_network(ip: &String) -> bool {
  for elem in RESERVED_RANGES_REGEX.iter() {
    if (**elem).is_match(ip.as_str()) {
      return true;
    }
  }
  return false;
}

static IP_RANGE_MIN: u32 = 0;
static IP_RANGE_MAX: u32 = 255;

static CIDR_MIN: u32 = 1;
static CIDR_MAX: u32 = 127;

lazy_static! {
  static ref REGEXP_PRIVATE_LOCAL: Regex = Regex::new(r"^10\.").unwrap();
  static ref REGEXP_PRIVATE_SHARED_ADDR: Regex = Regex::new(r"^100\.(6[4-9]|[7-9]\d|1[0-1]\d|12[0-7])\.").unwrap();
  static ref REGEXP_PRIVATE_LOOPBACK: Regex = Regex::new(r"^127\.").unwrap();
  static ref REGEXP_PRIVATE_LOCAL_LINK_ADDR: Regex = Regex::new(r"^169\.254\.").unwrap();
  static ref REGEXP_PRIVATE_LOCAL_PRIVATE_1: Regex = Regex::new(r"^172\.(1[6-9]|2\d|3[0-1])\.").unwrap();
  static ref REGEXP_PRIVATE_IETF_PROTOCOLS: Regex = Regex::new(r"^192\.0\.0\.").unwrap();
  static ref REGEXP_PRIVATE_LOCAL_PRIVATE_2: Regex = Regex::new(r"^192\.168\.").unwrap();
  static ref REGEXP_PRIVATE_BENCHMARK: Regex = Regex::new(r"^198\.(1[8-9])\.").unwrap();

  static ref PRIVATE_RANGES_REGEX: [&'static Regex; 8] = [
    &REGEXP_PRIVATE_LOCAL,
    &REGEXP_PRIVATE_SHARED_ADDR,
    &REGEXP_PRIVATE_LOOPBACK,
    &REGEXP_PRIVATE_LOCAL_LINK_ADDR,
    &REGEXP_PRIVATE_LOCAL_PRIVATE_1,
    &REGEXP_PRIVATE_IETF_PROTOCOLS,
    &REGEXP_PRIVATE_LOCAL_PRIVATE_2,
    &REGEXP_PRIVATE_BENCHMARK
  ];
  static ref PRIVATE_RANGES_REGEX_LEN: usize = PRIVATE_RANGES_REGEX.len();

  static ref REGEXP_RESERVED_LOCAL: Regex = Regex::new(r"^0\.").unwrap();
  static ref REGEXP_RESERVED_TEST_NET_1: Regex = Regex::new(r"^192\.0\.2\.").unwrap();
  static ref REGEXP_RESERVED_IPV6_RELAY: Regex = Regex::new(r"^192\.88\.99\.").unwrap();
  static ref REGEXP_RESERVED_TEST_NET_2: Regex = Regex::new(r"^198\.51\.100\.").unwrap();
  static ref REGEXP_RESERVED_TEST_NET_3: Regex = Regex::new(r"^203\.0\.113\.").unwrap();
  static ref REGEXP_RESERVED_MULTICAST: Regex = Regex::new(r"^(22[4-9]|23\d)\.").unwrap();
  static ref REGEXP_RESERVED_FUTURE_USE: Regex = Regex::new(r"^(24\d|25[0-5])\.").unwrap();

  static ref RESERVED_RANGES_REGEX: [&'static Regex; 7] = [
    &REGEXP_RESERVED_LOCAL,
    &REGEXP_RESERVED_TEST_NET_1,
    &REGEXP_RESERVED_IPV6_RELAY,
    &REGEXP_RESERVED_TEST_NET_2,
    &REGEXP_RESERVED_TEST_NET_3,
    &REGEXP_RESERVED_MULTICAST,
    &REGEXP_RESERVED_FUTURE_USE
  ];
  static ref RESERVED_RANGES_REGEX_LEN: usize = RESERVED_RANGES_REGEX.len();

  static ref PRIVATE_RANGES: [&'static [Range<i32>; 4]; 8] = [
    &[10..10, 0..255, 0..255, 1..255],
    &[100..100, 64..127, 0..255, 1..255],
    &[127..127, 0..255, 0..255, 1..255],
    &[169..169, 254..254, 0..255, 1..255],
    &[172..172, 16..31, 0..255, 1..255],
    &[192..192, 0..0, 0..0, 1..255],
    &[192..192, 168..168, 0..255, 1..255],
    &[198..198, 18..19, 0..255, 1..255]
  ];
  static ref PRIVATE_RANGES_LEN: usize = PRIVATE_RANGES.len();
}