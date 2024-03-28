use crate::utils::seeder;
use wasm_bindgen::prelude::*;

pub mod adjective;
pub mod adverb;
pub mod verb;

#[wasm_bindgen(js_name = zh_CN__word_adjective)]
pub fn adjective() -> String {
    use adjective::{ZH_CN__ADJECTIVE, ZH_CN__ADJECTIVE_LEN};
	ZH_CN__ADJECTIVE[seeder::gen_range(0..ZH_CN__ADJECTIVE_LEN)].to_string()
}

#[wasm_bindgen(js_name = zh_CN__word_adverb)]
pub fn adverb() -> String {
    use adverb::{ZH_CN__ADVERB, ZH_CN__ADVERB_LEN};
	ZH_CN__ADVERB[seeder::gen_range(0..ZH_CN__ADVERB_LEN)].to_string()
}

#[wasm_bindgen(js_name = zh_CN__word_verb)]
pub fn verb() -> String {
    use verb::{ZH_CN__VERB, ZH_CN__VERB_LEN};
	ZH_CN__VERB[seeder::gen_range(0..ZH_CN__VERB_LEN)].to_string()
}
