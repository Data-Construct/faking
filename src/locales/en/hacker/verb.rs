use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_verb)]
pub fn verb() -> String {
	EN_VERB[seeder::gen_range(0..EN_VERB_LEN)].to_string()
}

static EN_VERB: [&'static str; 18] = [
    "back up",
    "bypass",
    "hack",
    "override",
    "compress",
    "copy",
    "navigate",
    "index",
    "connect",
    "generate",
    "quantify",
    "calculate",
    "synthesize",
    "input",
    "transmit",
    "program",
    "reboot",
    "parse",
];
static EN_VERB_LEN: usize = EN_VERB.len();