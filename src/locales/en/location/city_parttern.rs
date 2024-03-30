use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::city_name;
use super::city_prefix;
use super::city_suffix;
use crate::locales::en::person::name;


#[wasm_bindgen(js_name = location_city_parttern)]
pub fn city_parttern() -> String {
	let format = seeder::gen_range(0..10);

	match format {
		0 => city_prefix::city_prefix() + " " + &name::male_first_name() + " " + &city_suffix::city_suffix(),
		1 => city_prefix::city_prefix() + " " + &name::female_first_name() + " " + &city_suffix::city_suffix(),
		2 => city_prefix::city_prefix() + " " + &name::male_first_name(),
		3 => city_prefix::city_prefix() + " " + &name::female_first_name(),
		4 => name::male_first_name() + " " + &city_suffix::city_suffix(),
        5 => name::female_first_name() + " " + &city_suffix::city_suffix(),
        6 => name::last_name() + " " + &city_suffix::city_suffix(),
        7 => name::last_name() + " " + &city_suffix::city_suffix(),
        _ => city_name::city_name(),
	}
}