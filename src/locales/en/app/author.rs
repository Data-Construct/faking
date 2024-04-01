use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use crate::locales::en::person::name;
use crate::locales::en::company::name_pattern::name_pattern;

fn person_name() -> String {
    let format = seeder::gen_range(0..2);

	match format {
		0 => name::male_first_name() + " " + &name::last_name(),
        1 => name::female_first_name() + " " + &name::last_name(),
        _ => "".to_string(),
	}
}

#[wasm_bindgen(js_name = app_author)]
pub fn author() -> String {
    let format = seeder::gen_range(0..2);

	match format {
		0 => person_name(),
        1 => name_pattern(),
        _ => "".to_string(),
	}
}


