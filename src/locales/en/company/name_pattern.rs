use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use crate::locales::en::person::name;
use super::suffix::suffix;

#[wasm_bindgen(js_name = company_name_pattern)]
pub fn name_pattern() -> String {
	let format = seeder::gen_range(0..3);

	// repeat the line to equal to probability
	match format {
		0 => name::last_name() + " " + &suffix(), // '{{person.last_name}} {{company.suffix}}'
        1 => name::last_name() + " - " + &name::last_name(), // '{{person.last_name}} - {{person.last_name}}'
        2 => name::last_name() + ", " + &name::last_name() + " and " + &name::last_name(), // '{{person.last_name}}, {{person.last_name}} and {{person.last_name}}'
		_ => "".to_string(),
	}
}