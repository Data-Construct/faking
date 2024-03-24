use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mac_address(prefix: Option<String>) -> String {
  let mut mac_builder: Vec<i32> = vec![];
  if let Some(arg) = prefix {
    let tokens = arg.split(":");
    for t in tokens {
      let mut i: i32 = i32::from_str_radix(t, 16).unwrap();

      if i > MAC_RANGE_MAX {
        i = MAC_RANGE_MAX;
      }
      mac_builder.push(i);
    }
  }

  for _ in 0..(MAC_SEGMENT_NUM-mac_builder.len()) {
    mac_builder.push(seeder::gen_range(MAC_RANGE_MIN..MAC_RANGE_MAX));
  }

  let mut mac_tokens: Vec<String> = vec![];
  for i in mac_builder {
    mac_tokens.push(format!("{:x}", i));
  }
  let s = mac_tokens.join(":");
  s
}

static MAC_RANGE_MIN: i32 = 0;
static MAC_RANGE_MAX: i32 = 256;
static MAC_SEGMENT_NUM: usize = 6;