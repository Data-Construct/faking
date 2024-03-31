use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn longitude() -> String {
	let mut degrees: f32 = seeder::gen_range(-90.0..90.0);
	let mut dir = 'N';
	if degrees < 0.0 {
		dir = 'S';
		degrees = degrees * -1.0;
	}

	format!("{:.7}°{}", degrees, dir)
}

#[wasm_bindgen]
pub fn latitude() -> String {
	let mut degrees: f32 = seeder::gen_range(-180.0..180.0);
	let mut dir = 'E';
	if degrees < 0.0 {
		dir = 'W';
		degrees = degrees * -1.0;
	}

	format!("{:.7}°{}", degrees, dir)
}

#[wasm_bindgen]
pub fn coordinates() -> String {
	let lon = longitude();
	let lat = latitude();
	concat_string!(lon, " ", lat)
}
