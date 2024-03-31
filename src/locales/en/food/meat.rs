use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = food_meat)]
pub fn meat() -> String {
	EN_MEAT[seeder::gen_range(0..EN_MEAT_LEN)].to_string()
}

static EN_MEAT: [&'static str; 16] = [
    "beef",
    "chicken",
    "crocodile",
    "duck",
    "emu",
    "goose",
    "kangaroo",
    "lamb",
    "ostrich",
    "pigeon",
    "pork",
    "quail",
    "rabbit",
    "salmon",
    "turkey",
    "venison",
];
static EN_MEAT_LEN: usize = EN_MEAT.len();