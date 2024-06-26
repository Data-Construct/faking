use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_cetacean)]
pub fn cetacean() -> String {
	EN_CETACEAN[seeder::gen_range(0..EN_CETACEAN_LEN)].to_string()
}

static EN_CETACEAN: [&'static str; 54] = [
  "Blue Whale",
  "Fin Whale",
  "Sei Whale",
  "Sperm Whale",
  "Brydes whale",
  "Omuras whale",
  "Humpback whale",
  "Long-Beaked Common Dolphin",
  "Short-Beaked Common Dolphin",
  "Bottlenose Dolphin",
  "Indo-Pacific Bottlenose Dolphin",
  "Northern Rightwhale Dolphin",
  "Southern Rightwhale Dolphin",
  "Tucuxi",
  "Costero",
  "Indo-Pacific Hump-backed Dolphin",
  "Chinese White Dolphin",
  "Atlantic Humpbacked Dolphin",
  "Atlantic Spotted Dolphin",
  "Clymene Dolphin",
  "Pantropical Spotted Dolphin",
  "Spinner Dolphin",
  "Striped Dolphin",
  "Rough-Toothed Dolphin",
  "Chilean Dolphin",
  "Commersons Dolphin",
  "Heavisides Dolphin",
  "Hectors Dolphin",
  "Rissos Dolphin",
  "Frasers Dolphin",
  "Atlantic White-Sided Dolphin",
  "Dusky Dolphin",
  "Hourglass Dolphin",
  "Pacific White-Sided Dolphin",
  "Peales Dolphin",
  "White-Beaked Dolphin",
  "Australian Snubfin Dolphin",
  "Irrawaddy Dolphin",
  "Melon-headed Whale",
  "Killer Whale (Orca)",
  "Pygmy Killer Whale",
  "False Killer Whale",
  "Long-finned Pilot Whale",
  "Short-finned Pilot Whale",
  "Guiana Dolphin",
  "Burrunan Dolphin",
  "Australian humpback Dolphin",
  "Amazon River Dolphin",
  "Chinese River Dolphin",
  "Ganges River Dolphin",
  "La Plata Dolphin",
  "Southern Bottlenose Whale",
  "Longmans Beaked Whale",
  "Arnouxs Beaked Whale",
];
static EN_CETACEAN_LEN: usize = EN_CETACEAN.len();