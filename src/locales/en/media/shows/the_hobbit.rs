use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn the_hobbit_characters() -> String {
  CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn the_hobbit_thorins_company() -> String {
  THORINS_COMPANY[seeder::gen_range(0..THORINS_COMPANY_LEN)].to_string()
}

#[wasm_bindgen]
pub fn the_hobbit_locations() -> String {
  LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn the_hobbit_quotes() -> String {
  QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

static CHARACTERS: [&'static str; 36] = [
  "Bilbo Baggins",
  "Bungo Baggins",
  "Belladonna Took",
  "Bullroarer Took",
  "Gandalf The Grey",
  "Radagast",
  "Dain",
  "Thorin Oakenshield",
  "Fili",
  "Kili",
  "Balin",
  "Dwalin",
  "Oin",
  "Gloin",
  "Dori",
  "Nori",
  "Ori",
  "Bifur",
  "Bofur",
  "Bombur",
  "Elrond",
  "Galion",
  "Bard the Bowman",
  "Beorn",
  "Tom",
  "Bert",
  "Bill Huggins",
  "Gollum",
  "The Necromancer",
  "Smaug",
  "Carc",
  "Roac",
  "The Lord of the Eagles",
  "The Great Goblin",
  "Bolg",
  "Golfimbul"
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static THORINS_COMPANY: [&'static str; 15] = [
  "Thorin Oakenshield",
  "Fili",
  "Kili",
  "Balin",
  "Dwalin",
  "Oin",
  "Gloin",
  "Dori",
  "Nori",
  "Ori",
  "Bifur",
  "Bofur",
  "Bombur",
  "Gandalf",
  "Bilbo Baggins"
];
static THORINS_COMPANY_LEN: usize = THORINS_COMPANY.len();

static LOCATIONS: [&'static str; 38] = [
  "Bree",
  "The Shire",
  "Rivendell",
  "The Misty Mountains",
  "Beorns Hall",
  "Mirkwood",
  "Esgaroth",
  "Erebor",
  "Bag-End",
  "Under-Hill",
  "Mount Gram",
  "Green Fields",
  "Last Desert",
  "Lonely Mountain",
  "Withered Heath",
  "Country Round",
  "Long Lake",
  "River Running",
  "Mines of Moria",
  "Green Dragon Inn",
  "Bywater",
  "The Great Mill",
  "Wilderland",
  "Gondolin",
  "Land Beyond",
  "Goblin Gate",
  "Carrock",
  "High Pass",
  "Great River",
  "Grey Mountains",
  "Land of the Necromancer",
  "Long Marshes",
  "Forest River",
  "Lake Town",
  "Dorwinion",
  "Ravenhill",
  "Iron Hills",
  "Mount Gundabad"
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 12] = [
  "Do you wish me a good morning, or mean that it is a good morning whether I want it or not; or that you feel good this morning; or that it is a morning to be good on?",
  "There is nothing like looking, if you want to find something. You certainly usually find something, if you look, but it is not always quite the something you were after.",
  "In a hole in the ground there lived a hobbit.",
  "It does not do to leave a live dragon out of your calculations, if you live near him.",
  "May the wind under your wings bear you where the sun sails and the moon walks.",
  "Where theres life theres hope.",
  "So comes snow after fire, and even dragons have their endings.",
  "Where did you go to, if I may ask? said Thorin to Gandalf as they rode along. To look ahead said he. And what brought you back in the nick of time? Looking behind said he.",
  "You have nice manners for a thief and a liar, said the dragon.",
  "May the hair on your toes never fall out!",
  "The road goes ever on and on...",
  "Never laugh at live dragons, Bilbo you fool!"
];
static QUOTES_LEN: usize = QUOTES.len();