use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_dog)]
pub fn dog() -> String {
	ZH_CN__DOG[seeder::gen_range(0..ZH_CN__DOG_LEN)].to_string()
}

static ZH_CN__DOG: [&'static str; 30] = [
    "藏獒",
    "袖狗",
    "拉萨狮子犬",
    "西藏狮子犬",
    "松狮犬",
    "中国冠毛犬",
    "西施犬",
    "沙皮犬",
    "八哥犬",
    "西藏獚",
    "中华田园犬",
    "下司犬",
    "北京犬",
    "西藏梗",
    "柴犬",
    "哈士奇",
    "德国牧羊犬",
    "边境牧羊犬",
    "贵兵犬",
    "秋田犬",
    "罗威纳犬",
    "蝴蝶犬",
    "英国斗牛犬",
    "阿富汗猎犬",
    "萨摩耶犬",
    "大白熊犬",
    "比利时牧羊犬",
    "美国爱斯基摩犬",
    "彭布罗克威尔士柯基犬",
    "墨西哥无毛犬",
];
static ZH_CN__DOG_LEN: usize = ZH_CN__DOG.len();