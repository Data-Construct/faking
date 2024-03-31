use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ko_word_noun)]
pub fn noun() -> String {
	KO_NOUN[seeder::gen_range(0..KO_NOUN_LEN)].to_string()
}

static KO_NOUN: [&'static str; 109] = [
    "가입",
    "경보",
    "고뇌",
    "고도",
    "고문",
    "골목",
    "공군",
    "공기",
    "공적",
    "공항",
    "관리",
    "관리자",
    "광고",
    "구급차",
    "구실",
    "나이",
    "내세",
    "놀람",
    "농업",
    "달력",
    "대사",
    "대수학",
    "대안",
    "대행사",
    "도토리",
    "동맹",
    "동작",
    "무정부주의자",
    "반침",
    "배당",
    "배우",
    "변경",
    "별명",
    "보적",
    "보좌관",
    "부사",
    "분석",
    "분석자",
    "분위기",
    "불쌍한",
    "비행기",
    "비행사",
    "비행선",
    "비행장",
    "성인",
    "성취",
    "성취자",
    "쇠붙이",
    "술",
    "승인",
    "아드레날린",
    "아마존",
    "아몬드",
    "악어",
    "안건",
    "알파벳",
    "알파카",
    "애프터셰이브",
    "앨범",
    "양",
    "양자",
    "어댑터",
    "에어백",
    "에이전트",
    "에일",
    "여배우",
    "여진",
    "여파",
    "연산",
    "예의",
    "오후",
    "옹호",
    "외계인",
    "용돈",
    "유연",
    "유추",
    "음향학",
    "응집",
    "이점",
    "일",
    "적응",
    "전능자",
    "전진",
    "제단",
    "조언",
    "조정",
    "주소",
    "즐거움",
    "지원",
    "진보",
    "진술",
    "진술서",
    "출현",
    "침략",
    "탄약",
    "탐닉",
    "특사",
    "합금",
    "합의",
    "항공기",
    "항공료",
    "항공편",
    "해석학",
    "행동",
    "형용사",
    "호박색",
    "활동",
    "활동가",
    "활성화",
];
static KO_NOUN_LEN: usize = KO_NOUN.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = noun();
// 		assert!(KO_NOUN.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 109;
// 		assert!(KO_NOUN.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
