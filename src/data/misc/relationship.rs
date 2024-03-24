use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn familial_direct() -> String {
	FAMILIAL_DIRECT[seeder::gen_range(0..FAMILIAL_DIRECT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn familial_extended() -> String {
	FAMILIAL_EXTENDED[seeder::gen_range(0..FAMILIAL_EXTENDED_LEN)].to_string()
}

#[wasm_bindgen]
pub fn in_law() -> String {
	IN_LAW[seeder::gen_range(0..IN_LAW_LEN)].to_string()
}

#[wasm_bindgen]
pub fn spouse() -> String {
	SPOUSE[seeder::gen_range(0..SPOUSE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn parent() -> String {
	PARENT[seeder::gen_range(0..PARENT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn sibling() -> String {
	SIBLING[seeder::gen_range(0..SIBLING_LEN)].to_string()
}

static FAMILIAL_DIRECT: [&'static str; 4] = [
	"Father",
	"Mother",
	"Sister",
	"Brother",
];
static FAMILIAL_DIRECT_LEN: usize = FAMILIAL_DIRECT.len();

static FAMILIAL_EXTENDED: [&'static str; 9] = [
	"Grandfather",
	"Grandmother",
	"Uncle",
	"Aunt",
	"Cousin",
	"Niece",
	"Nephew",
	"Grandson",
	"Granddaughter",
];
static FAMILIAL_EXTENDED_LEN: usize = FAMILIAL_EXTENDED.len();

static IN_LAW: [&'static str; 4] = [
	"Father-in-law",
	"Mother-in-law",
	"Sister-in-law",
	"Brother-in-law",
];
static IN_LAW_LEN: usize = IN_LAW.len();

static SPOUSE: [&'static str; 2] = [
	"Husband",
	"Wife",
];
static SPOUSE_LEN: usize = SPOUSE.len();

static PARENT: [&'static str; 2] = [
	"Father",
	"Mother",
];
static PARENT_LEN: usize = PARENT.len();

static SIBLING: [&'static str; 2] = [
	"Brother",
	"Sister",
];
static SIBLING_LEN: usize = SIBLING.len();
