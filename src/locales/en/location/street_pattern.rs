use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::street_name::street_name;
use super::street_suffix::street_suffix;
use crate::locales::en::person::name;

#[wasm_bindgen(js_name = location_street_pattern)]
pub fn street_parttern() -> String {
	let format = seeder::gen_range(0..3);

	// repeat the line to equal to probability
	match format {
		0 => name::first_name() + " " + &street_suffix(),
        1 => name::last_name() + " " + &street_suffix(),
		2 => street_name(),
		_ => "".to_string(),
	}
}