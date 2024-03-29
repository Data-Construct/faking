use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_bear)]
pub fn bear() -> String {
	ZH_CN__BEAR[seeder::gen_range(0..ZH_CN__BEAR_LEN)].to_string()
}

static ZH_CN__BEAR: [&'static str; 8] = [
    "大熊猫",
    "眼镜熊",
    "太阳熊",
    "懒熊",
    "美洲黑熊",
    "亚洲黑熊",
    "棕熊",
    "北极熊",
];
static ZH_CN__BEAR_LEN: usize = ZH_CN__BEAR.len();