use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn longitude() -> String {
	let mut degrees = seeder::gen_range(-90..91);
	let mut dir = 'N';
	if degrees < 0 {
		dir = 'S';
		degrees = degrees * -1;
	}
	let minutes = seeder::gen_range(0..61);
	let seconds: f32 = seeder::gen_range(0.0..60.0);

	format!("{}°{}'{:.4}\\\"{}", degrees, minutes, seconds, dir)
}

#[wasm_bindgen]
pub fn latitude() -> String {
	let mut degrees = seeder::gen_range(-180..181);
	let mut dir = 'E';
	if degrees < 0 {
		dir = 'W';
		degrees = degrees * -1;
	}
	let minutes = seeder::gen_range(0..61);
	let seconds: f32 = seeder::gen_range(0.0..60.0);

	format!("{}°{}'{:.4}\\\"{}", degrees, minutes, seconds, dir)
}

#[wasm_bindgen]
pub fn coordinates() -> String {
	let lon = longitude();
	let lat = latitude();
	concat_string!(lon, " ", lat)
}
