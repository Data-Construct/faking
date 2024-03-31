use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = color_human)]
pub fn human() -> String {
	EN_HUMAN[seeder::gen_range(0..EN_HUMAN_LEN)].to_string()
}

static EN_HUMAN: [&'static str; 31] = [
    "red",
    "green",
    "blue",
    "yellow",
    "purple",
    "mint green",
    "teal",
    "white",
    "black",
    "orange",
    "pink",
    "grey",
    "maroon",
    "violet",
    "turquoise",
    "tan",
    "sky blue",
    "salmon",
    "plum",
    "orchid",
    "olive",
    "magenta",
    "lime",
    "ivory",
    "indigo",
    "gold",
    "fuchsia",
    "cyan",
    "azure",
    "lavender",
    "silver",
];
static EN_HUMAN_LEN: usize = EN_HUMAN.len();