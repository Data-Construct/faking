use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ean_8() -> String {
  get_ean(CONST_EAN_8)
}

#[wasm_bindgen]
pub fn ean_8_with_composite_symbology() -> String {
  get_ean_with_composite_symbology(CONST_EAN_8)
}

#[wasm_bindgen]
pub fn ean_13() -> String {
  get_ean(CONST_EAN_13)
}

#[wasm_bindgen]
pub fn ean_13_with_composite_symbology(_l: i64) -> String {
  get_ean_with_composite_symbology(CONST_EAN_13)
}

fn get_ean(l: i64) -> String {
  let mut s: String = get_numeric_string_of_len(l-1);
  append_check_digit(&mut s);
  s
}

fn get_ean_with_composite_symbology(l: i64) -> String {
  let mut eanstr = get_ean(l);

  let symbol = get_composite_symbol();
  let cstr = get_composite_string(symbol);
  eanstr.push_str("|");
  eanstr.push_str(&cstr);
  eanstr
}

#[wasm_bindgen]
pub fn upc_a() -> String {
  let mut s: String = get_numeric_string_of_len(11);
  append_check_digit(&mut s);
  s
}

#[wasm_bindgen]
pub fn upc_a_with_composite_symbology() -> String {
  let mut astr = upc_a();

  let symbol = get_composite_symbol();
  let cstr = get_composite_string(symbol);
  astr.push_str("|");
  astr.push_str(&cstr);
  astr
}

#[wasm_bindgen]
pub fn upc_e() -> String {
  let first: i64 = seeder::gen_range(0..1);

  let mut s: String = first.to_string();
  s.push_str(&get_numeric_string_of_len(8));
  append_check_digit(&mut s);
  s
}

#[wasm_bindgen]
pub fn upc_e_with_composite_symbology() -> String {
  let mut estr = upc_e();

  let symbol = get_composite_symbol();
  let cstr = get_composite_string(symbol);
  estr.push_str("|");
  estr.push_str(&cstr);
  estr
}

#[wasm_bindgen]
pub fn isbn() -> String {
  let pre: String = ISBN_PREFIX[seeder::gen_range(0..ISBN_PREFIX_LEN)].to_string();

  let mut s: String = pre.clone();
  s.push_str(&get_numeric_string_of_len(CONST_ISBN_LEN-s.len() as i64));
  append_check_digit(&mut s);
  s
}

#[wasm_bindgen]
pub fn ismn() -> String {
  let mut s: String = "9790".to_owned();
  s.push_str(&get_numeric_string_of_len(CONST_ISMN_LEN));
  append_check_digit(&mut s);
  s
}

#[wasm_bindgen]
pub fn issn() -> String {
  let mut s: String = "977".to_owned();
  s.push_str(&get_numeric_string_of_len(CONST_ISSN_LEN));
  append_check_digit(&mut s);
  s
}

fn get_numeric_string_of_len(l: i64) -> String {
  let mut s: String = "".to_owned();

  for _ in 0..(l - 1) {
    let n = seeder::gen_range(0..10);
    s.push_str(&n.to_string());
  }
  s
}

fn append_check_digit(s: &mut String) {
  let tup = get_even_odd_sums(s.clone());
  let cd = get_check_digit(tup.0, tup.1);
  s.push_str(&cd.to_string());
}

fn get_composite_symbol() -> String {
  COMPOSITE_SYMBOLS[seeder::gen_range(0..COMPOSITE_SYMBOLS_LEN)].to_string()
}

fn get_composite_string(s: String) -> String {
  let mut comp: String = "".to_owned();
  for c in s.chars() {
    let mut tmp: String = "".to_owned();
    if c == CHAR_POUND {
      let n = seeder::gen_range(0..10);
      tmp = n.to_string();
    } else if c == CHAR_QUEST {
      tmp = get_upper_alpha();
    }
    comp.push_str(&tmp);
  }
  comp
}

fn get_upper_alpha() -> String {
  // A = 65..Z = 90
  let res: u8 = seeder::gen_range(65..91);
  let c = char::from(res);
  c.to_string()
}

fn get_even_odd_sums(s: String) -> (i64, i64) {
  let mut num: i64 = s.parse().unwrap();
  let mut even = 0;
  let mut odd = 0;
  let mut index = 1;

  while num != 0 {
    if is_even(index) {
      even += num % 10;
    } else {
      odd += num % 10;
    }
    num /= 10;
    index += 1;
  }
  (odd, even)
}

fn is_even(i: i64) -> bool {
  i&1 == 0
}

fn get_check_digit(odd_sum: i64, even_sum: i64) -> i64 {
  // Verified using: https://barcode-coder.com/en/ean-8-specification-101.html
  (10 - (odd_sum * 3 + even_sum) % 10) % 10
}

static CONST_ISBN_LEN: i64 = 13;
static CONST_ISMN_LEN: i64 = 8;
static CONST_ISSN_LEN: i64 = 9;
static CONST_EAN_8: i64 = 8;
static CONST_EAN_13: i64 = 13;
static CHAR_POUND: char = '#';
static CHAR_QUEST: char = '?';

static ISBN_PREFIX: [&'static str; 5] = [
  "978",
  "9798",
  "97910",
  "97911",
  "97912"
];
static ISBN_PREFIX_LEN: usize = ISBN_PREFIX.len();

static COMPOSITE_SYMBOLS: [&'static str; 6] = [
  "########",
  "????????",
  "####????",
  "????####",
  "##??##??",
  "??##??##"
];
static COMPOSITE_SYMBOLS_LEN: usize = COMPOSITE_SYMBOLS.len();