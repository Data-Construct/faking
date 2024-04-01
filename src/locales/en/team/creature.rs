use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = team_creature)]
pub fn creature() -> String {
	EN_CREATURE[seeder::gen_range(0..EN_CREATURE_LEN)].to_string()
}

static EN_CREATURE: [&'static str; 67] = [
    "ants",
    "bats",
    "bears",
    "bees",
    "birds",
    "buffalo",
    "cats",
    "chickens",
    "cattle",
    "dogs",
    "dolphins",
    "ducks",
    "elephants",
    "fishes",
    "foxes",
    "frogs",
    "geese",
    "goats",
    "horses",
    "kangaroos",
    "lions",
    "monkeys",
    "owls",
    "oxen",
    "penguins",
    "people",
    "pigs",
    "rabbits",
    "sheep",
    "tigers",
    "whales",
    "wolves",
    "zebras",
    "banshees",
    "crows",
    "black cats",
    "chimeras",
    "ghosts",
    "conspirators",
    "dragons",
    "dwarves",
    "elves",
    "enchanters",
    "exorcists",
    "sons",
    "foes",
    "giants",
    "gnomes",
    "goblins",
    "gooses",
    "griffins",
    "lycanthropes",
    "nemesis",
    "ogres",
    "oracles",
    "prophets",
    "sorcerors",
    "spiders",
    "spirits",
    "vampires",
    "warlocks",
    "vixens",
    "werewolves",
    "witches",
    "worshipers",
    "zombies",
    "druids",
];
static EN_CREATURE_LEN: usize = EN_CREATURE.len();

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = creature();
// 		assert!(EN_CREATURE.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 67;
// 		assert!(EN_CREATURE.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }