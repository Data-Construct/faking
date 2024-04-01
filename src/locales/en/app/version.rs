use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = app_version)]
pub fn version() -> String {
    let format = seeder::gen_range(0..5);

	match format {
		0 => format!("0.{}.{}", seeder::gen_range(0..10), seeder::gen_range(0..10)),
        1 => format!("0.{}{}", seeder::gen_range(0..10), seeder::gen_range(0..10)),
        2 => format!("{}.{}{}", seeder::gen_range(0..10), seeder::gen_range(0..10), seeder::gen_range(0..10)),
        3 => format!("{}.{}", seeder::gen_range(0..10), seeder::gen_range(0..10)),
        4 => format!("{}.{}.{}", seeder::gen_range(0..10), seeder::gen_range(0..10), seeder::gen_range(0..10)),
        _ => "".to_string(),
	}
}


