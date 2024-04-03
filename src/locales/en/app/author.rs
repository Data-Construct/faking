use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use crate::locales::en::person::name::name;
use crate::locales::en::company::name_pattern::name_pattern;

#[wasm_bindgen(js_name = app_author)]
pub fn author() -> String {
    let format = seeder::gen_range(0..2);

	match format {
		0 => name(),
        1 => name_pattern(),
        _ => "".to_string(),
	}
}



