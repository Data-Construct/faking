use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::city_name;
use super::city_prefix;
use super::city_suffix;
use crate::locales::en::person::name;


#[wasm_bindgen(js_name = location_city_pattern)]
pub fn city_parttern() -> String {
	let format = seeder::gen_range(0..5);

	match format {
		0 => city_prefix::city_prefix() + " " + &name::first_name() + &city_suffix::city_suffix(),
		1 => city_prefix::city_prefix() + " " + &name::first_name(),
		2 => name::first_name() + &city_suffix::city_suffix(),
        3 => name::last_name() + &city_suffix::city_suffix(),
        4 => city_name::city_name(),
		_ => "".to_string(),
	}
}