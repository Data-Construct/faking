use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pokemon_names() -> String {
	NAMES[seeder::gen_range(0..NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn pokemon_moves() -> String {
	MOVES[seeder::gen_range(0..MOVES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn pokemon_locations() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

static NAMES: [&'static str; 151] = [
	"Bulbasaur",
	"Ivysaur",
	"Venusaur",
	"Charmander",
	"Charmeleon",
	"Charizard",
	"Squirtle",
	"Wartortle",
	"Blastoise",
	"Caterpie",
	"Metapod",
	"Butterfree",
	"Weedle",
	"Kakuna",
	"Beedrill",
	"Pidgey",
	"Pidgeotto",
	"Pidgeot",
	"Rattata",
	"Raticate",
	"Spearow",
	"Fearow",
	"Ekans",
	"Arbok",
	"Pikachu",
	"Raichu",
	"Sandshrew",
	"Sandslash",
	"Nidoran",
	"Nidorina",
	"Nidoqueen",
	"Nidoran",
	"Nidorino",
	"Nidoking",
	"Clefairy",
	"Clefable",
	"Vulpix",
	"Ninetales",
	"Jigglypuff",
	"Wigglytuff",
	"Zubat",
	"Golbat",
	"Oddish",
	"Gloom",
	"Vileplume",
	"Paras",
	"Parasect",
	"Venonat",
	"Venomoth",
	"Diglett",
	"Dugtrio",
	"Meowth",
	"Persian",
	"Psyduck",
	"Golduck",
	"Mankey",
	"Primeape",
	"Growlithe",
	"Arcanine",
	"Poliwag",
	"Poliwhirl",
	"Poliwrath",
	"Abra",
	"Kadabra",
	"Alakazam",
	"Machop",
	"Machoke",
	"Machamp",
	"Bellsprout",
	"Weepinbell",
	"Victreebel",
	"Tentacool",
	"Tentacruel",
	"Geodude",
	"Graveler",
	"Golem",
	"Ponyta",
	"Rapidash",
	"Slowpoke",
	"Slowbro",
	"Magnemite",
	"Magneton",
	"Farfetchd",
	"Doduo",
	"Dodrio",
	"Seel",
	"Dewgong",
	"Grimer",
	"Muk",
	"Shellder",
	"Cloyster",
	"Gastly",
	"Haunter",
	"Gengar",
	"Onix",
	"Drowzee",
	"Hypno",
	"Krabby",
	"Kingler",
	"Voltorb",
	"Electrode",
	"Exeggcute",
	"Exeggutor",
	"Cubone",
	"Marowak",
	"Hitmonlee",
	"Hitmonchan",
	"Lickitung",
	"Koffing",
	"Weezing",
	"Rhyhorn",
	"Rhydon",
	"Chansey",
	"Tangela",
	"Kangaskhan",
	"Horsea",
	"Seadra",
	"Goldeen",
	"Seaking",
	"Staryu",
	"Starmie",
	"Mr. Mime",
	"Scyther",
	"Jynx",
	"Electabuzz",
	"Magmar",
	"Pinsir",
	"Tauros",
	"Magikarp",
	"Gyarados",
	"Lapras",
	"Ditto",
	"Eevee",
	"Vaporeon",
	"Jolteon",
	"Flareon",
	"Porygon",
	"Omanyte",
	"Omastar",
	"Kabuto",
	"Kabutops",
	"Aerodactyl",
	"Snorlax",
	"Articuno",
	"Zapdos",
	"Moltres",
	"Dratini",
	"Dragonair",
	"Dragonite",
	"Mewtwo",
	"Mew",
];
static NAMES_LEN: usize = NAMES.len();

static MOVES: [&'static str; 165] = [
	"Absorb",
	"Acid",
	"Acid Armor",
	"Agility",
	"Amnesia",
	"Aurora Beam",
	"Barrage",
	"Barrier",
	"Bide",
	"Bind",
	"Bite",
	"Blizzard",
	"Body Slam",
	"Bone Club",
	"Bonemerang",
	"Bubble",
	"Bubble Beam",
	"Clamp",
	"Comet Punch",
	"Confuse Ray",
	"Confusion",
	"Constrict",
	"Conversion",
	"Counter",
	"Crabhammer",
	"Cut",
	"Defense Curl",
	"Dig",
	"Disable",
	"Dizzy Punch",
	"Double Kick",
	"Double Slap",
	"Double Team",
	"Double-Edge",
	"Dragon Rage",
	"Dream Eater",
	"Drill Peck",
	"Earthquake",
	"Egg Bomb",
	"Ember",
	"Explosion",
	"Fire Blast",
	"Fire Punch",
	"Fire Spin",
	"Fissure",
	"Flamethrower",
	"Flash",
	"Fly",
	"Focus Energy",
	"Fury Attack",
	"Fury Swipes",
	"Glare",
	"Growl",
	"Growth",
	"Guillotine",
	"Gust",
	"Harden",
	"Haze",
	"Headbutt",
	"High Jump Kick",
	"Horn Attack",
	"Horn Drill",
	"Hydro Pump",
	"Hyper Beam",
	"Hyper Fang",
	"Hypnosis",
	"Ice Beam",
	"Ice Punch",
	"Jump Kick",
	"Karate Chop",
	"Kinesis",
	"Leech Life",
	"Leech Seed",
	"Leer",
	"Lick",
	"Light Screen",
	"Lovely Kiss",
	"Low Kick",
	"Meditate",
	"Mega Drain",
	"Mega Kick",
	"Mega Punch",
	"Metronome",
	"Mimic",
	"Minimize",
	"Mirror Move",
	"Mist",
	"Night Shade",
	"Pay Day",
	"Peck",
	"Petal Dance",
	"Pin Missile",
	"Poison Gas",
	"Poison Powder",
	"Poison Sting",
	"Pound",
	"Psybeam",
	"Psychic",
	"Psywave",
	"Quick Attack",
	"Rage",
	"Razor Leaf",
	"Razor Wind",
	"Recover",
	"Reflect",
	"Rest",
	"Roar",
	"Rock Slide",
	"Rock Throw",
	"Rolling Kick",
	"Sand Attack",
	"Scratch",
	"Screech",
	"Seismic Toss",
	"Self-Destruct",
	"Sharpen",
	"Sing",
	"Skull Bash",
	"Sky Attack",
	"Slam",
	"Slash",
	"Sleep Powder",
	"Sludge",
	"Smog",
	"Smokescreen",
	"Soft-Boiled",
	"Solar Beam",
	"Sonic Boom",
	"Spike Cannon",
	"Splash",
	"Spore",
	"Stomp",
	"Strength",
	"String Shot",
	"Struggle",
	"Stun Spore",
	"Submission",
	"Substitute",
	"Super Fang",
	"Supersonic",
	"Surf",
	"Swift",
	"Swords Dance",
	"Tackle",
	"Tail Whip",
	"Take Down",
	"Teleport",
	"Thrash",
	"Thunder",
	"Thunder Punch",
	"Thunder Shock",
	"Thunder Wave",
	"Thunderbolt",
	"Toxic",
	"Transform",
	"Tri Attack",
	"Twineedle",
	"Vice Grip",
	"Vine Whip",
	"Water Gun",
	"Waterfall",
	"Whirlwind",
	"Wing Attack",
	"Withdraw",
	"Wrap",
];
static MOVES_LEN: usize = MOVES.len();

static LOCATIONS: [&'static str; 98] = [
	"Accumula Town",
	"Ambrette Town",
	"Anistar City",
	"Anville Town",
	"Aquacorde Town",
	"Aspertia City",
	"Azalea Town",
	"Black City",
	"Blackthorn City",
	"Camphrier Town",
	"Canalave City",
	"Castelia City",
	"Celadon City",
	"Celestic Town",
	"Cerulean City",
	"Cherrygrove City",
	"Cianwood City",
	"Cinnabar Island",
	"Coumarine City",
	"Couriway Town",
	"Cyllage City",
	"Dendemille Town",
	"Dewford Town",
	"Driftveil City",
	"Ecruteak City",
	"Eterna City",
	"Ever Grande City",
	"Fallarbor Town",
	"Fight Area",
	"Five Island",
	"Floaroma Town",
	"Floccesy Town",
	"Fortree City",
	"Four Island",
	"Frontier Access",
	"Fuchsia City",
	"Geosenge Town",
	"Goldenrod City",
	"Hearthome City",
	"Humilau City",
	"Icirrus City",
	"Jubilife City",
	"Kiloude City",
	"Lacunosa Town",
	"Lavaridge Town",
	"Lavender Town",
	"Laverre City",
	"Lentimas Town",
	"Littleroot Town",
	"Lilycove City",
	"Lumiose City",
	"Mahogany Town",
	"Mauville City",
	"Mistralton City",
	"Mossdeep City",
	"Nacrene City",
	"New Bark Town",
	"Nimbasa City",
	"Nuvema Town",
	"Oldale Town",
	"Olivine City",
	"One Island",
	"Opelucid City",
	"Oreburgh City",
	"Pacifidlog Town",
	"Pallet Town",
	"Pastoria City",
	"Petalburg City",
	"Pewter City",
	"Resort Area",
	"Rustboro City",
	"Safari Zone Gate",
	"Saffron City",
	"Sandgem Town",
	"Santalune City",
	"Striaton City",
	"Seven Island",
	"Shalour City",
	"Six Island",
	"Slateport City",
	"Snowbelle City",
	"Snowpoint City",
	"Solaceon Town",
	"Sootopolis City",
	"Sunyshore City",
	"Survival Area",
	"Three Island",
	"Twinleaf Town",
	"Two Island",
	"Undella Town",
	"Vaniville Town",
	"Veilstone City",
	"Verdanturf Town",
	"Vermilion City",
	"Violet City",
	"Virbank City",
	"Viridian City",
	"White Forest",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();
