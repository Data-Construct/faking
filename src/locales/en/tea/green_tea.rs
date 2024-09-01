use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = tea_green_tea)]
pub fn green_tea() -> String {
	EN_GREEN_TEA[seeder::gen_range(0..EN_GREEN_TEA_LEN)].to_string()
}

static EN_GREEN_TEA: [&'static str; 23] = [
    "Bancha",
    "Biluochun",
    "Chun Mee",
    "Daejak",
    "Garucha",
    "Genmaicha",
    "Gunpowder",
    "Gyokuro",
    "Hojicha",
    "Huangshan Maofeng",
    "Ipcha",
    "Jungjak",
    "Kabusecha",
    "Kukicha",
    "Longjing",
    "Lu'an Melon Seed",
    "Matcha",
    "Sejak",
    "Sencha",
    "Shincha",
    "Taipin Houkui",
    "Ujeon",
    "Xinyang Maojian",
];
static EN_GREEN_TEA_LEN: usize = EN_GREEN_TEA.len();