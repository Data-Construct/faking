use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn god() -> String {
	GODS[seeder::gen_range(0..GODS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn primordial() -> String {
	PRIMORDIALS[seeder::gen_range(0..PRIMORDIALS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn titan() -> String {
	TITANS[seeder::gen_range(0..TITANS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn hero() -> String {
	HEROES[seeder::gen_range(0..HEROES_LEN)].to_string()
}

static GODS: [&'static str; 14] = [
	"Aphrodite",
	"Apollo",
	"Ares",
	"Artemis",
	"Athena",
	"Demeter",
	"Dionysus",
	"Hades",
	"Hephaestus",
	"Hera",
	"Hermes",
	"Hestia",
	"Poseidon",
	"Zeus",
];
static GODS_LEN: usize = GODS.len();

static PRIMORDIALS: [&'static str; 20] = [
	"Aion",
	"Aether",
	"Ananke",
	"Chaos",
	"Chronos",
	"Erebus",
	"Eros",
	"Hypnos",
	"Nesoi",
	"Uranus",
	"Gaia",
	"Ourea",
	"Phanes",
	"Pontus",
	"Tartarus",
	"Thalassa",
	"Thanatos",
	"Hemera",
	"Nyx",
	"Nemesis",
];
static PRIMORDIALS_LEN: usize = PRIMORDIALS.len();

static TITANS: [&'static str; 33] = [
	"Coeus",
	"Crius",
	"Cronus",
	"Hyperion",
	"Iapetus",
	"Mnemosyne",
	"Oceanus",
	"Phoebe",
	"Rhea",
	"Tethys",
	"Theia",
	"Themis",
	"Asteria",
	"Astraeus",
	"Atlas",
	"Aura",
	"Clymene",
	"Dione",
	"Helios",
	"Selene",
	"Eos",
	"Epimetheus",
	"Eurybia",
	"Eurynome",
	"Lelantos",
	"Leto",
	"Menoetius",
	"Metis",
	"Ophion",
	"Pallas",
	"Perses",
	"Prometheus",
	"Styx",
];
static TITANS_LEN: usize = TITANS.len();

static HEROES: [&'static str; 57] = [
	"Abderus",
	"Achilles",
	"Aeneas",
	"Ajax",
	"Amphitryon",
	"Antilochus",
	"Bellerophon",
	"Castor",
	"Chrysippus",
	"Daedalus",
	"Diomedes",
	"Eleusis",
	"Eunostus",
	"Ganymede",
	"Hector",
	"Hercules",
	"Icarus",
	"Iolaus",
	"Jason",
	"Meleager",
	"Odysseus",
	"Orpheus",
	"Pandion",
	"Perseus",
	"Theseus",
	"Alcestis",
	"Amymone",
	"Andromache",
	"Andromeda",
	"Antigone",
	"Arachne",
	"Ariadne",
	"Atalanta",
	"Briseis",
	"Caeneus",
	"Cassandra",
	"Cassiopeia",
	"Clytemnestra",
	"Danaë",
	"Deianeira",
	"Electra",
	"Europa",
	"Hecuba",
	"Helen",
	"Hermione",
	"Iphigenia",
	"Ismene",
	"Jocasta",
	"Medea",
	"Medusa",
	"Niobe",
	"Pandora",
	"Penelope",
	"Phaedra",
	"Polyxena",
	"Semele",
	"Thrace",
];
static HEROES_LEN: usize = HEROES.len();
