use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = color_human)]
pub fn department() -> String {
	EN_DEPARMENT[seeder::gen_range(0..EN_DEPARMENT_LEN)].to_string()
}

static EN_DEPARMENT: [&'static str; 22] = [
    "Books",
    "Movies",
    "Music",
    "Games",
    "Electronics",
    "Computers",
    "Home",
    "Garden",
    "Tools",
    "Grocery",
    "Health",
    "Beauty",
    "Toys",
    "Kids",
    "Baby",
    "Clothing",
    "Shoes",
    "Jewelery",
    "Sports",
    "Outdoors",
    "Automotive",
    "Industrial",
];
static EN_DEPARMENT_LEN: usize = EN_DEPARMENT.len();