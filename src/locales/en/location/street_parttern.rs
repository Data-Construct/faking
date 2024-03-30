use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::street_name::street_name;
use super::street_suffix::street_suffix;
use crate::locales::en::person::name;

#[wasm_bindgen(js_name = location_city_parttern)]
pub fn street_parttern() -> String {
	let format = seeder::gen_range(0..5);

	// repeat the line to equal to probability
	match format {
		0 => name::male_first_name() + " " + &street_suffix(),
        1 => name::female_first_name() + " " + &street_suffix(),
        2 => name::last_name() + " " + &street_suffix(),
        3 => name::last_name() + " " + &street_suffix(),
		_ => street_name(),
	}
}