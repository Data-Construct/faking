use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = internet_free_mail)]
pub fn free_mail() -> String {
	EN_FREE_MAIL[seeder::gen_range(0..EN_FREE_MAIL_LEN)].to_string()
}

static EN_FREE_MAIL: [&'static str; 3] = [
    "gmail.com", 
    "yahoo.com", 
    "hotmail.com", 
];
static EN_FREE_MAIL_LEN: usize = EN_FREE_MAIL.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = free_mail();
// 		assert!(EN_FREE_MAIL.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 3;
// 		assert!(EN_FREE_MAIL.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }