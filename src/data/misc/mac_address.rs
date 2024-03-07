use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mac_address(prefix: Option<String>) -> String {
  let mut macBuilder: Vec<i32> = vec![];
  if let Some(arg) = prefix {
    let tokens = arg.split(":");
    for t in tokens {
      let mut i: i32 = i32::from_str_radix(t, 16).unwrap();

      if i > MAC_RANGE_MAX {
        i = MAC_RANGE_MAX;
      }
      macBuilder.push(i);
    }
  }

  let mut rng = rand::thread_rng();
  for _ in 0..(MAC_SEGMENT_NUM-macBuilder.len()) {
    macBuilder.push(rng.gen_range(MAC_RANGE_MIN..MAC_RANGE_MAX));
  }

  let mut macTokens: Vec<String> = vec![];
  for i in macBuilder {
    macTokens.push(format!("{:x}", i));
  }
  let s = macTokens.join(":");
  s
}

static MAC_RANGE_MIN: i32 = 0;
static MAC_RANGE_MAX: i32 = 256;
static MAC_SEGMENT_NUM: usize = 6;