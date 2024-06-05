use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = tea_black_tea)]
pub fn black_tea() -> String {
	EN_BLACK_TEA[seeder::gen_range(0..EN_BLACK_TEA_LEN)].to_string()
}

static EN_BLACK_TEA: [&'static str; 24] = [
    "Assam",
    "Ceylon",
    "Congou",
    "Darjeeling",
    "Dianhong",
    "Earl Grey",
    "English Afternoon",
    "English Breakfast",
    "Irish Breakfast",
    "Jaekseol",
    "Jiu Qu Hong Mei",
    "Kangra",
    "Keemun",
    "Lady Grey",
    "Lahijan",
    "Lapsang Souchong",
    "Masala Chai",
    "Munnar",
    "Nepali",
    "Nilgiri",
    "Rize",
    "Scottish Breakfast",
    "Sun Moon Lake",
    "Yingdehong",
];
static EN_BLACK_TEA_LEN: usize = EN_BLACK_TEA.len();