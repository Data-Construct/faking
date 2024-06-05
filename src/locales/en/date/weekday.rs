use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = date_weekday_wide)]
pub fn weekday_wide() -> String {
	EN_WEEKDAY_WIDE[seeder::gen_range(0..EN_WEEKDAY_WIDE_LEN)].to_string()
}

static EN_WEEKDAY_WIDE: [&'static str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
static EN_WEEKDAY_WIDE_LEN: usize = EN_WEEKDAY_WIDE.len();

#[wasm_bindgen(js_name = date_weekday_abbr)]
pub fn weekday_abbr() -> String {
	EN_WEEKDAY_ABBR[seeder::gen_range(0..EN_WEEKDAY_ABBR_LEN)].to_string()
}

static EN_WEEKDAY_ABBR: [&'static str; 7] = [
    "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat",
];
static EN_WEEKDAY_ABBR_LEN: usize = EN_WEEKDAY_ABBR.len();

