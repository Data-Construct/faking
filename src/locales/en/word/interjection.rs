use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = word_interjection)]
pub fn interjection() -> String {
	EN_INTERJECTION[seeder::gen_range(0..EN_INTERJECTION_LEN)].to_string()
}

static EN_INTERJECTION: [&'static str; 46] = [
  "yuck",
  "oh",
  "phooey",
  "blah",
  "boo",
  "whoa",
  "yowza",
  "huzzah",
  "boohoo",
  "fooey",
  "geez",
  "pfft",
  "ew",
  "ah",
  "yum",
  "brr",
  "hm",
  "yahoo",
  "aha",
  "woot",
  "drat",
  "gah",
  "meh",
  "psst",
  "aw",
  "ugh",
  "yippee",
  "eek",
  "gee",
  "bah",
  "gadzooks",
  "duh",
  "ha",
  "mmm",
  "ouch",
  "phew",
  "ack",
  "uh-huh",
  "gosh",
  "hmph",
  "pish",
  "zowie",
  "er",
  "ick",
  "oof",
  "um",
];
static EN_INTERJECTION_LEN: usize = EN_INTERJECTION.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = interjection();
// 		assert!(EN_INTERJECTION.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 46;
// 		assert!(EN_INTERJECTION.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
