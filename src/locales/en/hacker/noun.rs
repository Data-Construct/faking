use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_noun)]
pub fn noun() -> String {
	EN_NOUN[seeder::gen_range(0..EN_NOUN_LEN)].to_string()
}

static EN_NOUN: [&'static str; 24] = [
    "driver",
    "protocol",
    "bandwidth",
    "panel",
    "microchip",
    "program",
    "port",
    "card",
    "array",
    "interface",
    "system",
    "sensor",
    "firewall",
    "hard drive",
    "pixel",
    "alarm",
    "feed",
    "monitor",
    "application",
    "transmitter",
    "bus",
    "circuit",
    "capacitor",
    "matrix",
];
static EN_NOUN_LEN: usize = EN_NOUN.len();