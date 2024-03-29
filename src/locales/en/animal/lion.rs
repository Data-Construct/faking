use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_lion)]
pub fn lion() -> String {
	EN__LION[seeder::gen_range(0..EN__LION_LEN)].to_string()
}

static EN__LION: [&'static str; 7] = [
    "Asiatic Lion",
    "Barbary Lion",
    "West African Lion",
    "Northeast Congo Lion",
    "Masai Lion",
    "Transvaal lion",
    "Cape lion",
];
static EN__LION_LEN: usize = EN__LION.len();