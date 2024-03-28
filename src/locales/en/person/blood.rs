use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn blood_type() -> String {
	BLOOD_TYPES[seeder::gen_range(0..BLOOD_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn rh_factory() -> String {
	RH_FACTORS[seeder::gen_range(0..RH_FACTORS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn group() -> String {
	let blood_type = blood_type();
	let rh_factor = rh_factory();
	return concat_string!(blood_type, rh_factor);
}

static BLOOD_TYPES: [&'static str; 4] = [
	"O", "A", "B", "AB",
];
static BLOOD_TYPES_LEN: usize = BLOOD_TYPES.len();

static RH_FACTORS: [&'static str; 2] = [
	"+", "-",
];
static RH_FACTORS_LEN: usize = RH_FACTORS.len();
