use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = internet_domain_suffix)]
pub fn domain_suffix() -> String {
	EN_DOMAIN_SUFFIX[seeder::gen_range(0..EN_DOMAIN_SUFFIX_LEN)].to_string()
}

static EN_DOMAIN_SUFFIX: [&'static str; 6] = [
    "com", 
    "biz", 
    "info", 
    "name", 
    "net", 
    "org",
];
static EN_DOMAIN_SUFFIX_LEN: usize = EN_DOMAIN_SUFFIX.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = domain_suffix();
// 		assert!(EN_DOMAIN_SUFFIX.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 6;
// 		assert!(EN_DOMAIN_SUFFIX.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }