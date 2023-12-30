use wasm_bindgen::prelude::*;
use rand::distributions::{Alphanumeric, DistString};

#[wasm_bindgen]
pub fn generate_wallet_address() -> String {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 40);
    format!("0x{}", string.to_lowercase())
}