use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_crocodilia)]
pub fn crocodilia() -> String {
	EN_CROCODILIA[seeder::gen_range(0..EN_CROCODILIA_LEN)].to_string()
}

static EN_CROCODILIA: [&'static str; 24] = [
  "Alligator mississippiensis",
  "Chinese Alligator",
  "Black Caiman",
  "Broad-snouted Caiman",
  "Spectacled Caiman",
  "Yacare Caiman",
  "Cuvier's Dwarf Caiman",
  "Schneider's Smooth-fronted Caiman",
  "African Slender-snouted Crocodile",
  "American Crocodile",
  "Australian Freshwater Crocodile",
  "Cuban Crocodile",
  "Dwarf Crocodile",
  "Morelet's Crocodile",
  "Mugger Crocodile",
  "New Guinea Freshwater Crocodile",
  "Nile Crocodile",
  "West African Crocodile",
  "Orinoco Crocodile",
  "Philippine Crocodile",
  "Saltwater Crocodile",
  "Siamese Crocodile",
  "Gharial",
  "Tomistoma",
];
static EN_CROCODILIA_LEN: usize = EN_CROCODILIA.len();
