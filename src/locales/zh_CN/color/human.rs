use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_HUMAN)]
pub fn human() -> String {
	ZH_CN__HUMAN[seeder::gen_range(0..ZH_CN__HUMAN_LEN)].to_string()
}

static ZH_CN__HUMAN: [&'static str; 26] = [
    "红色",
    "绿色",
    "蓝色",
    "黄色",
    "紫色",
    "薄荷绿色",
    "蓝绿色",
    "白色",
    "黑色",
    "橙色",
    "粉红色",
    "灰色",
    "红褐色",
    "蓝紫色",
    "青绿色",
    "棕褐色",
    "天蓝色",
    "浅橙色",
    "紫红色",
    "淡紫色",
    "淡褐色",
    "青柠色",
    "乳白色",
    "靛蓝色",
    "金色",
    "银色",
];
static ZH_CN__HUMAN_LEN: usize = ZH_CN__HUMAN.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = human();
// 		assert!(ZH_CN__HUMAN.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 26;
// 		assert!(ZH_CN__HUMAN.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }