use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_CN_hacker_noun)]
pub fn noun() -> String {
	ZH_CN_NOUN[seeder::gen_range(0..ZH_CN_NOUN_LEN)].to_string()
}

static ZH_CN_NOUN: [&'static str; 24] = [
    "驱动",
    "协议",
    "带宽",
    "面板",
    "芯片",
    "程序",
    "端口",
    "卡片",
    "数组",
    "接口",
    "系统",
    "传感器",
    "防火墙",
    "硬盘",
    "像素",
    "警报",
    "提要",
    "监视器",
    "应用",
    "发送端",
    "总线",
    "电路",
    "电容器",
    "矩阵",
];
static ZH_CN_NOUN_LEN: usize = ZH_CN_NOUN.len();

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_item_is_in_list() {
		let result = noun();
		assert!(ZH_CN_NOUN.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
	}

	#[test]
	fn test_len() {
		let length = 24;
		assert!(ZH_CN_NOUN.len() == length, "The length of the list is not equal to the setting.");
	}

	 
}