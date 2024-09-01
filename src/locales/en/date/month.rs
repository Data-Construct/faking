use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = date_month_wide)]
pub fn month_wide() -> String {
	EN_MONTH_WIDE[seeder::gen_range(0..EN_MONTH_WIDE_LEN)].to_string()
}

static EN_MONTH_WIDE: [&'static str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
static EN_MONTH_WIDE_LEN: usize = EN_MONTH_WIDE.len();

#[wasm_bindgen(js_name = date_month_abbr)]
pub fn month_abbr() -> String {
	EN_MONTH_ABBR[seeder::gen_range(0..EN_MONTH_ABBR_LEN)].to_string()
}

static EN_MONTH_ABBR: [&'static str; 12] = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];
static EN_MONTH_ABBR_LEN: usize = EN_MONTH_ABBR.len();