use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_horse)]
pub fn horse() -> String {
	ZH_CN__HORSSE[seeder::gen_range(0..ZH_CN__HORSSE_LEN)].to_string()
}

static ZH_CN__HORSSE: [&'static str; 4] = [
   "蒙古马 ",
   "伊利马 ", 
   "三河马 ", 
   "河曲马"
];
static ZH_CN__HORSSE_LEN: usize = ZH_CN__HORSSE.len();