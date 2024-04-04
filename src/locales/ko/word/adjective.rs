use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ko_word_adjective)]
pub fn adjective() -> String {
	KO_ADJECTIVE[seeder::gen_range(0..KO_ADJECTIVE_LEN)].to_string()
}

static KO_ADJECTIVE: [&'static str; 205] = [
    "감정적인",
    "같은",
    "거대한",
    "거창한",
    "건조한",
    "겁 없는",
    "격렬한",
    "결정적인",
    "경솔한",
    "경험한",
    "고귀한",
    "고급의",
    "고대의",
    "공정한",
    "관심 있는",
    "굉장한",
    "교양 있는",
    "교육받은",
    "교활한",
    "구부러진",
    "굴곡진",
    "굵은",
    "권위 있는",
    "귀여운",
    "극적인",
    "금발의",
    "기민한",
    "기분 좋은",
    "기쁜",
    "기초적인",
    "깊은",
    "깨끗한",
    "깨진",
    "끊임없는",
    "끔찍한",
    "나쁜",
    "날씬한",
    "냉담한",
    "넓은 마음을 가진",
    "놀라운",
    "눈부신",
    "눈이 먼",
    "늙은",
    "능숙한",
    "다른",
    "단조로운",
    "단호한",
    "닫힌",
    "당황스러운",
    "대담한",
    "대량",
    "더러운",
    "동굴 같은",
    "두려운",
    "뛰어난",
    "마른",
    "막대한",
    "맛있는",
    "매력적인",
    "매혹적인",
    "먹을 수 있는",
    "먼",
    "멍든",
    "메마른",
    "명확한",
    "모범적인",
    "무더운",
    "무서운",
    "무심한",
    "미친",
    "밀집한",
    "밝은",
    "방어",
    "방음",
    "버려진",
    "별개의",
    "복잡한",
    "부끄러운",
    "부담스러운",
    "부드러운",
    "부러워하는",
    "부정한",
    "부족한",
    "분명한",
    "분주한",
    "불결한",
    "불룩한",
    "불안한",
    "불충실한",
    "붐비는",
    "비뚤어진",
    "비싼",
    "비어 있는",
    "비참한",
    "빠른",
    "사랑스러운",
    "사랑하는",
    "사려 깊은",
    "사악한",
    "살아 있는",
    "상세한",
    "상쾌한",
    "생기 있는",
    "생분해성",
    "성실한",
    "세련된",
    "소름 끼치는",
    "솔직한",
    "순수한",
    "쉬운",
    "습한",
    "시원한",
    "신나는",
    "신뢰할 수 있는",
    "싼",
    "아름다운",
    "알고 있는",
    "약간의",
    "어느",
    "어두운",
    "어려운",
    "어리석은",
    "어색한",
    "어설픈",
    "어지러운",
    "억센",
    "엄청난",
    "역겨운",
    "열심히",
    "영리한",
    "예술적인",
    "예의 바른",
    "온화한",
    "완벽한",
    "외향적인",
    "용감한",
    "용기 있는",
    "우아한",
    "원통형",
    "위독한",
    "윙윙",
    "유리한",
    "유명한",
    "유익한",
    "유치한",
    "윤리적",
    "음침한",
    "의기 양양한",
    "의식하는",
    "이국적인",
    "이타적인",
    "인기 많은",
    "인정 많은",
    "일찍",
    "자신 있는",
    "잔혹한",
    "저명한",
    "저주받은",
    "적극적인",
    "적절한",
    "전통적인",
    "젊은",
    "정교한",
    "정통한",
    "정확한",
    "조잡한",
    "존경하는",
    "주의 깊은",
    "죽은",
    "즐거운",
    "지루한",
    "진지한",
    "짧은",
    "차가운",
    "창의적인",
    "철저한",
    "추운",
    "충실한",
    "치명적인",
    "친숙한",
    "친절한",
    "침착한",
    "쾌활한",
    "큰",
    "타원형의",
    "탄력 있는",
    "탈진한",
    "탐욕스러운",
    "통통한",
    "편안한",
    "품위 있는",
    "풍부한",
    "필수적인",
    "행복한",
    "향긋한",
    "혼란스러운",
    "화난",
    "화려한",
    "환상적",
    "활동적인",
    "활발한",
    "훌륭한",
    "흔한",
    "흥분한",
    "희미한",
];
static KO_ADJECTIVE_LEN: usize = KO_ADJECTIVE.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = adjective();
// 		assert!(KO_ADJECTIVE.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 205;
// 		assert!(KO_ADJECTIVE.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }