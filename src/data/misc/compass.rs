use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cardinal_words() -> String {
  let rng = rand::thread_rng();
  CARDINAL_WORDS[rng.gen_range(0..CARDINAL_LEN)];
}

#[wasm_bindgen]
pub fn cardinal_abbreviation() -> String {
  let rng = rand::thread_rng();
  CARDINAL_ABBREVIATION[rng.gen_range(0..CARDINAL_LEN)];
}

#[wasm_bindgen]
pub fn cardinal_azimuth() -> String {
  let rng = rand::thread_rng();
  CARDINAL_AZIMUTH[rng.gen_range(0..CARDINAL_LEN)];
}

#[wasm_bindgen]
pub fn ordinal_words() -> String {
  let rng = rand::thread_rng();
 ORDINAL_WORDS[rng.gen_range(0..ORDINAL_LEN)];
}

#[wasm_bindgen]
pub fn ordinal_abbreviation() -> String {
  let rng = rand::thread_rng();
  ORDINAL_ABBREVIATION[rng.gen_range(0..ORDINAL_LEN)];
}

#[wasm_bindgen]
pub fn ordinal_azimuth() -> String {
  let rng = rand::thread_rng();
  ORDINAL_AZIMUTH[rng.gen_range(0..ORDINAL_LEN)];
}

#[wasm_bindgen]
pub fn half_wind_words() -> String {
  let rng = rand::thread_rng();
  HALF_WIND_WORDS[rng.gen_range(0..HALF_WIND_LEN)];
}

#[wasm_bindgen]
pub fn half_wind_abbreviation() -> String {
  let rng = rand::thread_rng();
  HALF_WIND_ABBREVIATION[rng.gen_range(0..HALF_WIND_LEN)];
}

#[wasm_bindgen]
pub fn half_wind_azimuth() -> String {
  let rng = rand::thread_rng();
  HALF_WIND_AZIMUTH[rng.gen_range(0..HALF_WIND_LEN)];
}

#[wasm_bindgen]
pub fn quarter_wind_words() -> String {
  let rng = rand::thread_rng();
  QUARTER_WIND_WORDS[rng.gen_range(0..QUARTER_WIND_LEN)];
}

#[wasm_bindgen]
pub fn quarter_wind_abbreviation() -> String {
  let rng = rand::thread_rng();
  QUARTER_WIND_ABBREVIATION[rng.gen_range(0..QUARTER_WIND_LEN)];
}

#[wasm_bindgen]
pub fn quarter_wind_azimuth() -> String {
  let rng = rand::thread_rng();
  QUARTER_WIND_AZIMUTH[rng.gen_range(0..QUARTER_WIND_LEN)];
}

static CARDINAL_WORDS: [&'static str; 4] = [
  "north",
  "east",
  "south",
  "west"
];
static CARDINAL_LEN: usize = CARDINAL_WORDS.len();

static CARDINAL_ABBREVIATION: [&'static str; 4] = [
  "N",
  "E",
  "S",
  "W"
];

static CARDINAL_AZIMUTH: [&'static str; 4] = [
  "0",
  "90",
  "180",
  "270"
];

static ORDINAL_WORDS: [&'static str; 4] = [
  "northeast",
  "southeast",
  "southwest",
  "northwest"
];
static ORDINAL_LEN: usize = ORDINAL_WORDS.len();

static ORDINAL_ABBREVIATION: [&'static str; 4] = [
  "NE",
  "SE",
  "SW",
  "NW"
];

static ORDINAL_AZIMUTH: [&'static str; 4] = [
  "45",
  "135",
  "225",
  "315"
];

static HALF_WIND_WORDS: [&'static str; 8] = [
  "north-northeast",
  "east-northeast",
  "east-southeast",
  "south-southeast",
  "south-southwest",
  "west-southwest",
  "west-northwest",
  "north-northwest"
];
static HALF_WIND_LEN: usize = HALF_WIND_WORDS.len();


static HALF_WIND_ABBREVIATION: [&'static str; 4] = [
  "NNE",
  "ENE",
  "ESE",
  "SSE",
  "SSW",
  "WSW",
  "WNW",
  "NNW"
];

static HALF_WIND_AZIMUTH: [&'static str; 4] = [
  "22.5",
  "67.5",
  "112.5",
  "157.5",
  "202.5",
  "247.5",
  "292.5",
  "337.5"
];

static QUARTER_WIND_WORDS: [&'static str; 16] = [
  "north by east",
  "northeast by north",
  "northeast by east",
  "east by north",
  "east by south",
  "southeast by east",
  "southeast by south",
  "south by east",
  "south by west",
  "southwest by south",
  "southwest by west",
  "west by south",
  "west by north",
  "northwest by west",
  "northwest by north",
  "north by west"
];
static QUARTER_WIND_LEN: usize = QUARTER_WIND_WORDS.len();

static QUARTER_WIND_ABBREVIATION: [&'static str; 16] = [
  "NbE",
  "NEbN",
  "NEbE",
  "EbN",
  "EbS",
  "SEbE",
  "SEbS",
  "SbE",
  "SbW",
  "SWbS",
  "SWbW",
  "WbS",
  "WbN",
  "NWbW",
  "NWbN",
  "NbW"
];

static QUARTER_WIND_AZIMUTH: [&'static str; 16] = [
  "11.25",
  "33.75",
  "56.25",
  "78.75",
  "101.25",
  "123.75",
  "146.25",
  "168.75",
  "191.25",
  "213.75",
  "236.25",
  "258.75",
  "281.25",
  "303.75",
  "326.25",
  "348.75"
];