pub mod aircraft_type;
pub mod airlines;
use crate::utils::seeder;
use wasm_bindgen::prelude::*;

/// Returns a random aircraft type.
#[wasm_bindgen]
pub fn aircraft_type() -> String {
	use aircraft_type::{EN_AIRCRAFT_TYPE, EN_AIRCRAFT_TYPE_LEN};
	EN_AIRCRAFT_TYPE[seeder::gen_range(0..EN_AIRCRAFT_TYPE_LEN)].to_string()
}

/// Returns a random airline.
#[wasm_bindgen]
pub fn airline() -> String {
	use airlines::AIRLINE_HASHMAP;
	let random_number = seeder::gen_range(0..AIRLINE_HASHMAP.keys().len());
	AIRLINE_HASHMAP
		.get(&random_number)
		.unwrap()
		.get(0)
		.unwrap()
		.to_string()
}

/// Returns a random airline iata code.
#[wasm_bindgen]
pub fn airline_iata_code() -> String {
	use airlines::AIRLINE_HASHMAP;
	let random_number = seeder::gen_range(0..AIRLINE_HASHMAP.keys().len());
	AIRLINE_HASHMAP
		.get(&random_number)
		.unwrap()
		.get(1)
		.unwrap()
		.to_string()
}
