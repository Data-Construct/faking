use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = tea_oolong_tea)]
pub fn oolong_tea() -> String {
	EN_OOLONG_TEA[seeder::gen_range(0..EN_OOLONG_TEA_LEN)].to_string()
}

static EN_OOLONG_TEA: [&'static str; 20] = [
    "Alishan",
    "Bai Jiguan",
    "Da Hong Pao",
    "Dancong",
    "Dongding",
    "Dongfang Meiren",
    "Fujian",
    "Gaoshan",
    "Huangjin Gui",
    "Ji Xuan",
    "Lishan",
    "Pouchong",
    "Rougui",
    "Ruan Zhi",
    "Shui Jin Gui",
    "Shui Xian",
    "Tieguanyin",
    "Tieluohan",
    "Tienguanyin",
    "Vietnamese",
];
static EN_OOLONG_TEA_LEN: usize = EN_OOLONG_TEA.len();