use wasm_bindgen::prelude::*;
use crate::utils::seeder;

#[wasm_bindgen]
pub fn unix_ts() -> String {
  let (secs, nanos) = unix_ts_gen();
  concat_string!(secs.to_string(), nanos.to_string())
}

// Unix TS example:         1711702567
// Unix TS example (nanos): 1711702550856745973
pub fn unix_ts_gen() -> (u64, u32) {
  // Unix Timestamps only go to 2147483647 in most systems.
  //  We can arbitrarily increase this, if required.
  let secs: u64 = seeder::gen_range(0..=2_147_483_647);
  let nanos: u32 = seeder::gen_range(0..1_000_000_000);
  (secs, nanos)
}
