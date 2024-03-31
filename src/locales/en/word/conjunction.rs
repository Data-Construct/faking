use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = word_conjunction)]
pub fn conjunction() -> String {
	EN_CONJUNCTION[seeder::gen_range(0..EN_CONJUNCTION_LEN)].to_string()
}

static EN_CONJUNCTION: [&'static str; 51] = [
  "after",
  "although",
  "and",
  "as",
  "because",
  "before",
  "but",
  "consequently",
  "even",
  "finally",
  "for",
  "furthermore",
  "hence",
  "how",
  "however",
  "if",
  "inasmuch",
  "incidentally",
  "indeed",
  "instead",
  "lest",
  "likewise",
  "meanwhile",
  "nor",
  "now",
  "once",
  "or",
  "provided",
  "since",
  "so",
  "supposing",
  "than",
  "that",
  "though",
  "till",
  "unless",
  "until",
  "what",
  "when",
  "whenever",
  "where",
  "whereas",
  "wherever",
  "whether",
  "which",
  "while",
  "who",
  "whoever",
  "whose",
  "why",
  "yet",
];
static EN_CONJUNCTION_LEN: usize = EN_CONJUNCTION.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = conjunction();
// 		assert!(EN_CONJUNCTION.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 51;
// 		assert!(EN_CONJUNCTION.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
