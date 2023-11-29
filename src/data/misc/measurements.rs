use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn height_measurement() -> String {
	let mut rng = rand::thread_rng();
	HEIGHT[rng.gen_range(0..HEIGHT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn length_measurement() -> String {
	let mut rng = rand::thread_rng();
	LENGTH[rng.gen_range(0..LENGTH_LEN)].to_string()
}

#[wasm_bindgen]
pub fn volume_measurement() -> String {
	let mut rng = rand::thread_rng();
	VOLUME[rng.gen_range(0..VOLUME_LEN)].to_string()
}

#[wasm_bindgen]
pub fn weight_measurement() -> String {
	let mut rng = rand::thread_rng();
	WEIGHT[rng.gen_range(0..WEIGHT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn metric_height_measurement() -> String {
	let mut rng = rand::thread_rng();
	METRIC_HEIGHT[rng.gen_range(0..METRIC_HEIGHT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn metric_length_measurement() -> String {
	let mut rng = rand::thread_rng();
	METRIC_LENGTH[rng.gen_range(0..METRIC_LENGTH_LEN)].to_string()
}

#[wasm_bindgen]
pub fn metric_volume_measurement() -> String {
	let mut rng = rand::thread_rng();
	METRIC_VOLUME[rng.gen_range(0..METRIC_VOLUME_LEN)].to_string()
}

#[wasm_bindgen]
pub fn metric_weight_measurement() -> String {
	let mut rng = rand::thread_rng();
	METRIC_WEIGHT[rng.gen_range(0..METRIC_WEIGHT_LEN)].to_string()
}
static HEIGHT: [&'static str; 2] = [
	"inch",
	"foot",
];
static HEIGHT_LEN: usize = HEIGHT.len();

static LENGTH: [&'static str; 3] = [
	"yard",
	"foot",
	"mile",
];
static LENGTH_LEN: usize = LENGTH.len();

static VOLUME: [&'static str; 7] = [
	"cup",
	"tablespoon",
	"teaspoon",
	"quart",
	"pint",
	"gallon",
	"fluid ounce",
];
static VOLUME_LEN: usize = VOLUME.len();

static WEIGHT: [&'static str; 3] = [
	"pound",
	"ounce",
	"ton",
];
static WEIGHT_LEN: usize = WEIGHT.len();

static METRIC_HEIGHT: [&'static str; 2] = [
	"centimeter",
	"meter",
];
static METRIC_HEIGHT_LEN: usize = METRIC_HEIGHT.len();

static METRIC_LENGTH: [&'static str; 7] = [
	"millimeter",
	"centimeter",
	"decimeter",
	"meter",
	"dekameter",
	"hectometer",
	"kilometer",
];
static METRIC_LENGTH_LEN: usize = METRIC_LENGTH.len();

static METRIC_VOLUME: [&'static str; 2] = [
	"milliliter",
	"liter",
];
static METRIC_VOLUME_LEN: usize = METRIC_VOLUME.len();

static METRIC_WEIGHT: [&'static str; 8] = [
	"milligram",
	"centigram",
	"decigram",
	"gram",
	"dekagram",
	"hectogram",
	"kilogram",
	"metric ton",
];
static METRIC_WEIGHT_LEN: usize = METRIC_WEIGHT.len();
