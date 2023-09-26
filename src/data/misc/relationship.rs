use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn familial_direct() -> String {
	let mut rng = rand::thread_rng();
	FAMILIAL_DIRECT[rng.gen_range(0..FAMILIAL_DIRECT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn familial_extended() -> String {
	let mut rng = rand::thread_rng();
	FAMILIAL_EXTENDED[rng.gen_range(0..FAMILIAL_EXTENDED_LEN)].to_string()
}

#[wasm_bindgen]
pub fn in_law() -> String {
	let mut rng = rand::thread_rng();
	IN_LAW[rng.gen_range(0..IN_LAW_LEN)].to_string()
}

#[wasm_bindgen]
pub fn spouse() -> String {
	let mut rng = rand::thread_rng();
	SPOUSE[rng.gen_range(0..SPOUSE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn parent() -> String {
	let mut rng = rand::thread_rng();
	PARENT[rng.gen_range(0..PARENT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn sibling() -> String {
	let mut rng = rand::thread_rng();
	SIBLING[rng.gen_range(0..SIBLING_LEN)].to_string()
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
