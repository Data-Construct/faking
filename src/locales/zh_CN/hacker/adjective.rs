use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_hacker_adjective)]
pub fn adjective() -> String {
	ZH_CN_ADJECTIVE[seeder::gen_range(0..ZH_CN_ADJECTIVE_LEN)].to_string()
}

static ZH_CN_ADJECTIVE: [&'static str; 17] = [
    "辅助",
    "主要",
    "后端",
    "开源",
    "虚拟",
    "跨平台",
    "冗余",
    "在线",
    "触控",
    "多字节",
    "蓝牙",
    "无线",
    "全高清",
    "神经元",
    "光学",
    "固态",
    "移动",
];
static ZH_CN_ADJECTIVE_LEN: usize = ZH_CN_ADJECTIVE.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = ADJECTIVE();
// 		assert!(ZH_CN_ADJECTIVE.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 17;
// 		assert!(ZH_CN_ADJECTIVE.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }