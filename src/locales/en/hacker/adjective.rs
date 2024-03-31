use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_adjective)]
pub fn adjective() -> String {
	EN_ADJECTIVE[seeder::gen_range(0..EN_ADJECTIVE_LEN)].to_string()
}

static EN_ADJECTIVE: [&'static str; 18] = [
    "auxiliary",
    "primary",
    "back-end",
    "digital",
    "open-source",
    "virtual",
    "cross-platform",
    "redundant",
    "online",
    "haptic",
    "multi-byte",
    "bluetooth",
    "wireless",
    "1080p",
    "neural",
    "optical",
    "solid state",
    "mobile",
];
static EN_ADJECTIVE_LEN: usize = EN_ADJECTIVE.len();