use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mario_character() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn mario_game() -> String {
	GAMES[seeder::gen_range(0..GAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn mario_location() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

static CHARACTERS: [&'static str; 23] = [
	"Mario",
	"Luigi",
	"Princess Peach",
	"Toad",
	"Bowser",
	"Yoshi",
	"Princess Daisy",
	"Rosalina",
	"Donkey Kong",
	"Diddy Kong",
	"Toadette",
	"Birdo",
	"Toadsworth",
	"Captain Toad",
	"Pauline",
	"Wario",
	"Waluigi",
	"Bowser Jr.",
	"Koopalings",
	"Kamek",
	"Boom Boom",
	"Pom Pom",
	"King Boo",
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static GAMES: [&'static str; 11] = [
	"Luigi's Mansion",
	"Super Mario Bros.",
	"Super Mario World",
	"Super Mario Kart",
	"Super Mario 64",
	"Super Mario Party",
	"Super Smash Bros.",
	"Super Mario Galaxy",
	"Super Mario Odyssey",
	"Super Mario Maker",
	"Paper Mario",
];
static GAMES_LEN: usize = GAMES.len();

static LOCATIONS: [&'static str; 17] = [
	"Bonneton",
	"Fossil Falls",
	"Tostarena",
	"Lake Lamode",
	"Steam Gardens",
	"Nimbus Arena",
	"Forgoten Isle",
	"Kong City",
	"Shiveria",
	"Bubblaine",
	"Mount Volbono",
	"Crumbleden",
	"Bowser's Castle",
	"Honeylune Ridge",
	"Peach's Castle",
	"Rabbit Ridge",
	"Culmina Crater",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();
