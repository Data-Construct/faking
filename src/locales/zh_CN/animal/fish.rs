use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_fish)]
pub fn fish() -> String {
	ZH_CN__FISH[seeder::gen_range(0..ZH_CN__FISH_LEN)].to_string()
}

static ZH_CN__FISH: [&'static str; 11] = [
    "草鱼",
    "鲶鱼",
    "鳙鱼",
    "鲤鱼",
    "金鱼",
    "胭脂鱼",
    "中华鲟",
    "长江白鲟",
    "新疆大头鱼",
    "青鱼",
    "鲫鱼"
];
static ZH_CN__FISH_LEN: usize = ZH_CN__FISH.len();