use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lovecraft_deity() -> String {
	let mut rng = rand::thread_rng();
	DEITIES[rng.gen_range(0..DEITIES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lovecraft_book() -> String {
	let mut rng = rand::thread_rng();
	BOOKS[rng.gen_range(0..BOOKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lovecraft_location() -> String {
	let mut rng = rand::thread_rng();
	LOCATIONS[rng.gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lovecraft_artefact() -> String {
	let mut rng = rand::thread_rng();
	ARTEFACTS[rng.gen_range(0..ARTEFACTS_LEN)].to_string()
}

static DEITIES: [&'static str; 8] = [
    "Azathoth",
    "Cthulhu",
    "Dagon",
    "Hastur",
    "Nyarlathotep",
    "Shub-Niggurath",
    "Tsathoggua",
    "Yog-Sothoth",
];
static DEITIES_LEN: usize = DEITIES.len();

static BOOKS: [&'static str; 9] = [
    "Book of Azathoth",
    "Book of Eibon",
    "Cultes des Goules",
    "Dhol Chants",
    "Necronomicon",
    "Pnakotic Manuscripts",
    "Poakotic Fragments",
    "De Vermis Mysteriis",
    "Eltdown Shards",
];
static BOOKS_LEN: usize = BOOKS.len();

static LOCATIONS: [&'static str; 16] = [
    "Arkham",
    "Dunwich",
    "Innsmouth",
    "Kadath",
    "Kingsport",
    "Leng",
    "Miskatonic",
    "R'lyeh",
    "Yuggoth",
    "Irem",
    "Yaddith",
    "Camorin",
    "Carcosa",
    "Egyptica",
    "Oriab",
    "Sarkomand",

];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static ARTEFACTS: [&'static str; 16] = [
    "Black Fan",
    "Cobra Crown",
    "Crystallizers of Dreams",
    "Curse Whistle",
    "Elder Sign",
    "Golden mead",
    "Hound Amulet",
    "Lagh metal",
    "Liao",
    "Shard of Panestes",
    "Shining Trapezohedron",
    "Silver Key",
    "Tulu-metal",
    "Ultimate Gate",
    "Yellow Sign",
    "Yikilth",
];
static ARTEFACTS_LEN: usize = ARTEFACTS.len();