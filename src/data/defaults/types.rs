use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<bool>()
}
