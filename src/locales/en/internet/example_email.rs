use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = internet_example_email)]
pub fn example_email() -> String {
	EN_EXAMPLE_EMAIL[seeder::gen_range(0..EN_EXAMPLE_EMAIL_LEN)].to_string()
}

static EN_EXAMPLE_EMAIL: [&'static str; 3] = [
    "example.org", 
    "example.com", 
    "example.net", 
];
static EN_EXAMPLE_EMAIL_LEN: usize = EN_EXAMPLE_EMAIL.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = example_email();
// 		assert!(EN_EXAMPLE_EMAIL.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 3;
// 		assert!(EN_EXAMPLE_EMAIL.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }