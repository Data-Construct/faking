use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = tea_white_tea)]
pub fn white_tea() -> String {
	EN_WHITE_TEA[seeder::gen_range(0..EN_WHITE_TEA_LEN)].to_string()
}

static EN_WHITE_TEA: [&'static str; 5] = [
    "Bai Mu Dan",
    "Fujian New Craft",
    "Gongmei",
    "Shou Mei",
    "Yi Zhen Bai Hao",
];
static EN_WHITE_TEA_LEN: usize = EN_WHITE_TEA.len();