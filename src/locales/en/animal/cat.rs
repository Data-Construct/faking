use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = EN)]
pub fn cat() -> String {
	let mut rng = rand::thread_rng();
	EN__CAT[rng.gen_range(0..EN__CAT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn en_cat() -> String {
	let mut rng = rand::thread_rng();
	EN__CAT[rng.gen_range(0..EN__CAT_LEN)].to_string()
}

static EN__CAT: [&'static str; 55] = [
  "Abyssinian",
  "American Bobtail",
  "American Curl",
  "American Shorthair",
  "American Wirehair",
  "Balinese",
  "Bengal",
  "Birman",
  "Bombay",
  "British Shorthair",
  "Burmese",
  "Chartreux",
  "Chausie",
  "Cornish Rex",
  "Devon Rex",
  "Donskoy",
  "Egyptian Mau",
  "Exotic Shorthair",
  "Havana",
  "Highlander",
  "Himalayan",
  "Japanese Bobtail",
  "Korat",
  "Kurilian Bobtail",
  "LaPerm",
  "Maine Coon",
  "Manx",
  "Minskin",
  "Munchkin",
  "Nebelung",
  "Norwegian Forest Cat",
  "Ocicat",
  "Ojos Azules",
  "Oriental",
  "Persian",
  "Peterbald",
  "Pixiebob",
  "Ragdoll",
  "Russian Blue",
  "Savannah",
  "Scottish Fold",
  "Selkirk Rex",
  "Serengeti",
  "Siberian",
  "Siamese",
  "Singapura",
  "Snowshoe",
  "Sokoke",
  "Somali",
  "Sphynx",
  "Thai",
  "Tonkinese",
  "Toyger",
  "Turkish Angora",
  "Turkish Van",
];
static EN__CAT_LEN: usize = EN__CAT.len();