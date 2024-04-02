use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_hacker_verb)]
pub fn verb() -> String {
	ZH_CN_VERB[seeder::gen_range(0..ZH_CN_VERB_LEN)].to_string()
}

static ZH_CN_VERB: [&'static str; 18] = [
    "备份",
    "绕过",
    "入侵",
    "覆盖",
    "压缩",
    "复制",
    "导航",
    "索引",
    "链接",
    "生成",
    "量化",
    "计算",
    "合成",
    "输入",
    "传输",
    "编程",
    "重启",
    "解析",
];
static ZH_CN_VERB_LEN: usize = ZH_CN_VERB.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = verb();
// 		assert!(ZH_CN_VERB.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 18;
// 		assert!(ZH_CN_VERB.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }