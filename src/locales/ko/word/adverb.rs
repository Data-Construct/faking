use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ko_word_adverb)]
pub fn adverb() -> String {
	KO_ADVERB[seeder::gen_range(0..KO_ADVERB_LEN)].to_string()
}

static KO_ADVERB: [&'static str; 45] = [
    "간단히",
    "거꾸로",
    "거만하게",
    "거의",
    "고요히",
    "고의로",
    "극진히",
    "깊이",
    "나중에",
    "노하여",
    "다소",
    "다채롭게",
    "대담하게",
    "대체로",
    "도전적으로",
    "똑똑하게",
    "마구",
    "맹목적으로",
    "면밀히",
    "명랑하게",
    "몹시",
    "바르게",
    "바쁘게",
    "밝게",
    "분명히",
    "비정상적으로",
    "빠르게",
    "심하게",
    "아름답게",
    "어색하게",
    "언제나",
    "열심히",
    "용감하게",
    "우연히",
    "유감스럽게",
    "의심스럽게",
    "자랑스럽게",
    "잔인하게",
    "즐겁게",
    "지속적으로",
    "천하게",
    "쿨하게",
    "행복하게",
    "흥미롭게",
    "희미하게",
];
static KO_ADVERB_LEN: usize = KO_ADVERB.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = adverb();
// 		assert!(KO_ADVERB.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 45;
// 		assert!(KO_ADVERB.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
