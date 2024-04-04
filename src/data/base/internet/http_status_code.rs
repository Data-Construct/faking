use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = internet_informational)]
pub fn informational() -> String {
	EN_INFORMATIONAL[seeder::gen_range(0..EN_INFORMATIONAL_LEN)].to_string()
}

#[wasm_bindgen(js_name = internet_success)]
pub fn success() -> String {
	EN_SUCCESS[seeder::gen_range(0..EN_SUCCESS_LEN)].to_string()
}

#[wasm_bindgen(js_name = internet_redirection)]
pub fn redirection() -> String {
	EN_REDIRECTION[seeder::gen_range(0..EN_REDIRECTION_LEN)].to_string()
}

#[wasm_bindgen(js_name = internet_clientError)]
pub fn clientError() -> String {
	EN_CLIENT_ERROR[seeder::gen_range(0..EN_CLIENT_ERROR_LEN)].to_string()
}

#[wasm_bindgen(js_name = internet_serverError)]
pub fn serverError() -> String {
	EN_SERVER_ERROR[seeder::gen_range(0..EN_SERVER_ERROR_LEN)].to_string()
}

static EN_INFORMATIONAL: [&'static str; 4] = [
  "100", "101", "102", "103",
];
static EN_INFORMATIONAL_LEN: usize = EN_INFORMATIONAL.len();

static EN_SUCCESS: [&'static str; 10] = [
    "200", "201", "202", "203", "204", "205", "206", "207", "208", "226",
];
static EN_SUCCESS_LEN: usize = EN_SUCCESS.len();

static EN_REDIRECTION: [&'static str; 9] = [
    "300", "301", "302", "303", "304", "305", "306", "307", "308"
];
static EN_REDIRECTION_LEN: usize = EN_REDIRECTION.len();

static EN_CLIENT_ERROR: [&'static str; 29] = [
    "400", "401", "402", "403", "404", "405", "406", "407", "408", "409", "410", "411", "412", "413", "414",
    "415", "416", "417", "418", "421", "422", "423", "424", "425", "426", "428", "429", "431", "451",
];
static EN_CLIENT_ERROR_LEN: usize = EN_CLIENT_ERROR.len();

static EN_SERVER_ERROR: [&'static str; 11] = [
    "500", "501", "502", "503", "504", "505", "506", "507", "508", "510", "511"
];
static EN_SERVER_ERROR_LEN: usize = EN_SERVER_ERROR.len();