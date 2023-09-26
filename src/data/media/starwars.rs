use rand::Rng;
use wasm_bindgen::prelude::*;

//TODO Ask again how to deal with quotes

#[wasm_bindgen]
pub fn starwars_character() -> String {
	let mut rng = rand::thread_rng();
	STARWARS_CHARACTERS[rng.gen_range(0..STARWARS_CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_squadron() -> String {
	let mut rng = rand::thread_rng();
	CALL_SQUADRONS[rng.gen_range(0..CALL_SQUADRONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_number() -> String {
	let mut rng = rand::thread_rng();
	CALL_NUMBERS[rng.gen_range(0..CALL_NUMBERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_sign() -> String {
	let call_num = call_number();
	let call_squad = call_squadron();
	return format!("{} {}", call_num, call_squad);
}

#[wasm_bindgen]
pub fn droid() -> String {
	let mut rng = rand::thread_rng();
	DROIDS[rng.gen_range(0..DROIDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn planet() -> String {
	let mut rng = rand::thread_rng();
	PLANETS[rng.gen_range(0..PLANETS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn species() -> String {
	let mut rng = rand::thread_rng();
	SPECIES[rng.gen_range(0..SPECIES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn vehicle() -> String {
	let mut rng = rand::thread_rng();
	VEHICLES[rng.gen_range(0..VEHICLES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn wookie_word() -> String {
	let mut rng = rand::thread_rng();
	WOOKIE_WORDS[rng.gen_range(0..WOOKIE_WORDS_LEN)].to_string()
}
static STARWARS_CHARACTERS: [&'static str; 60] = [
	"Padme Amidala",
	"Jar Jar Binks",
	"Borvo the Hutt",
	"Darth Caedus",
	"Boba Fett",
	"Jabba the Hutt",
	"Obi-Wan Kenobi",
	"Darth Maul",
	"Leia Organa",
	"Sheev Palpatine",
	"Kylo Ren",
	"Darth Sidious",
	"Anakin Skywalker",
	"Luke Skywalker",
	"Ben Solo",
	"Han Solo",
	"Darth Vader",
	"Watto",
	"Mace Windu",
	"Yoda",
	"Count Dooku",
	"Sebulba",
	"Qui-Gon Jinn",
	"Chewbacca",
	"Jango Fett",
	"Lando Calrissian",
	"Bail Organa",
	"Wedge Antilles",
	"Poe Dameron",
	"Ki-Adi-Mundi",
	"Nute Gunray",
	"Panaka",
	"Rune Haako",
	"Rey",
	"Finn",
	"Supreme Leader Snoke",
	"General Hux",
	"Admiral Ackbar",
	"Ahsoka Tano",
	"Asajj Ventress",
	"Bendu",
	"Captain Phasma",
	"Chirrut Imwe",
	"Ezra Bridger",
	"Galen Erso",
	"Grand Moff Tarkin",
	"Grand Admiral Thrawn",
	"Greedo",
	"Jyn Erso",
	"Lyra Erso",
	"Maz Kanata",
	"Mon Mothma",
	"Sabine Wren",
	"Saw Gerrera",
	"Savage Opress",
	"Shmi Skywalker",
	"Kanan Jarrus",
	"Hera Syndulla",
	"Rose Tico",
	"Vice Admiral Holdo",
];
static STARWARS_CHARACTERS_LEN: usize = STARWARS_CHARACTERS.len();

static CALL_SQUADRONS: [&'static str; 9] = [
	"Rogue",
	"Red",
	"Gray",
	"Green",
	"Blue",
	"Gold",
	"Black",
	"Yellow",
	"Phoenix",
];
static CALL_SQUADRONS_LEN: usize = CALL_SQUADRONS.len();

static CALL_NUMBERS: [&'static str; 6] = [
	"Leader",
	"1",
	"2",
	"3",
	"4",
	"5",
];
static CALL_NUMBERS_LEN: usize = CALL_NUMBERS.len();

static DROIDS: [&'static str; 31] = [
	"2-1B",
	"4-LOM",
	"ASP",
	"B2-RP",
	"B1",
	"BD-3000",
	"C1-10P",
	"FA-4",
	"GH-7",
	"GNK",
	"LM-432",
	"ID9",
	"11-4D",
	"2-1B",
	"327-T",
	"4-LOM",
	"B4-D4",
	"NR-N99",
	"C-3PO",
	"R2-D2",
	"BB-8",
	"R2-Q5",
	"Super Battle Droid",
	"Mouse Droid",
	"Droideka",
	"Buzz Droid",
	"Magnaguard",
	"Interrogation Droid",
	"Vulture Droid",
	"BB-9E",
	"K-2SO",
];
static DROIDS_LEN: usize = DROIDS.len();

static PLANETS: [&'static str; 35] = [
	"Alderaan",
	"Ahch-To",
	"Bespin",
	"Cantonica",
	"Coruscant",
	"Cloud City",
	"Crait",
	"DQar",
	"Dathomir",
	"Dagobah",
	"Death Star",
	"Eadu",
	"Endor",
	"Felucia",
	"Geonosis",
	"Hoth",
	"Hosnian Prime",
	"Jakku",
	"Jedha",
	"Kamino",
	"Kashyyyk",
	"Lothal",
	"Mandalore",
	"Mustafar",
	"Mygeeto",
	"Naboo",
	"Onderon",
	"Ryloth",
	"Scarif",
	"Starkiller Base",
	"Sullust",
	"Takodana",
	"Tatooine",
	"Utapau",
	"Yavin 4",
];
static PLANETS_LEN: usize = PLANETS.len();

static SPECIES: [&'static str; 15] = [
	"Ewok",
	"Hutt",
	"Gungan",
	"Ithorian",
	"Jawa",
	"Neimoidian",
	"Sullustan",
	"Wookiee",
	"Mon Calamari",
	"Bith",
	"Dathomirian",
	"Gamorreans",
	"Kaminoan",
	"Twi'lek",
	"Porg",
];
static SPECIES_LEN: usize = SPECIES.len();

static VEHICLES: [&'static str; 36] = [
	"V-Wing Fighter",
	"ATT Battle Tank",
	"Naboo N-1 Starfighter",
	"Republic Cruiser",
	"Naboo Royal Starship",
	"Gungan Bongo Submarine",
	"Flash Speeder",
	"Trade Federation Battleship",
	"Millennium Falcon",
	"Sith Infiltrator",
	"AT-ST Walker",
	"TIE Bomber",
	"Imperial Shuttle",
	"Sandcrawler",
	"TIE Interceptor",
	"Speeder Bike",
	"Death Star",
	"AT-AT Walker",
	"Imperial Star Destroyer",
	"X-Wing Fighter",
	"A-Wing Fighter",
	"GR-75 Transport",
	"Imperial Interdictor",
	"MTT",
	"Phantom II",
	"Republic Attack Gunship",
	"Rey's Speeder",
	"Ghost",
	"U-Wing",
	"Y-Wing Starfighter",
	"First Order TIE Fighter",
	"AT-M6 Walker",
	"First Order Dreadnought",
	"TIE Silencer",
	"Resistance Bomber",
	"Resistance Ski Speeder",
];
static VEHICLES_LEN: usize = VEHICLES.len();

static WOOKIE_WORDS: [&'static str; 22] = [
	"wyaaaaaa",
	"ruh",
	"huewaa",
	"muaa",
	"mumwa",
	"wua",
	"ga",
	"ma",
	"ahuma",
	"ooma",
	"youw",
	"kabukk",
	"wyogg",
	"gwyaaaag",
	"roooarrgh",
	"ur",
	"ru",
	"roo",
	"hnn-rowr",
	"yrroonn",
	"nng",
	"rarr",
];
static WOOKIE_WORDS_LEN: usize = WOOKIE_WORDS.len();
