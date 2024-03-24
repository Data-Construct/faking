use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn planet() -> String {
	PLANETS[seeder::gen_range(0..PLANETS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn moon() -> String {
	MOONS[seeder::gen_range(0..MOONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn galaxy() -> String {
	GALAXYS[seeder::gen_range(0..GALAXYS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn nebula() -> String {
	NEBULAS[seeder::gen_range(0..NEBULAS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn star_cluster() -> String {
	STAR_CLUSTERS[seeder::gen_range(0..STAR_CLUSTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn constellation() -> String {
	CONSTELLATIONS[seeder::gen_range(0..CONSTELLATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn star() -> String {
	STARS[seeder::gen_range(0..STARS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn agency() -> String {
	AGENCYS[seeder::gen_range(0..AGENCYS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn agency_abv() -> String {
	AGENCYS_ABV[seeder::gen_range(0..AGENCYS_ABV_LEN)].to_string()
}

#[wasm_bindgen]
pub fn nasa_space_craft() -> String {
	NASA_SPACE_CRAFTS[seeder::gen_range(0..NASA_SPACE_CRAFTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn company() -> String {
	COMPANYS[seeder::gen_range(0..COMPANYS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn distance_measurement() -> String {
	DISTANCE_MEASUREMENTS[seeder::gen_range(0..DISTANCE_MEASUREMENTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn meteorite() -> String {
	METEORITES[seeder::gen_range(0..METEORITES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn launch_vehicle() -> String {
	LAUNCH_VEHICLES[seeder::gen_range(0..LAUNCH_VEHICLES_LEN)].to_string()
}

static PLANETS: [&'static str; 8] = [
	"Mercury",
	"Venus",
	"Earth",
	"Mars",
	"Jupiter",
	"Saturn",
	"Uranus",
	"Neptune",
];
static PLANETS_LEN: usize = PLANETS.len();

static MOONS: [&'static str; 20] = [
	"Moon",
	"Luna",
	"Deimos",
	"Phobos",
	"Ganymede",
	"Callisto",
	"Io",
	"Europa",
	"Titan",
	"Rhea",
	"Iapetus",
	"Dione",
	"Tethys",
	"Hyperion",
	"Ariel",
	"Puck",
	"Oberon",
	"Umbriel",
	"Triton",
	"Proteus",
];
static MOONS_LEN: usize = MOONS.len();

static GALAXYS: [&'static str; 10] = [
	"Milky Way",
	"Andromeda",
	"Triangulum",
	"Whirlpool",
	"Blackeye",
	"Sunflower",
	"Pinwheel",
	"Hoags Object",
	"Centaurus A",
	"Messier 83",
];
static GALAXYS_LEN: usize = GALAXYS.len();

static NEBULAS: [&'static str; 8] = [
	"Lagoon Nebula",
	"Eagle Nebula",
	"Triffid Nebula",
	"Dumbell Nebula",
	"Orion Nebula",
	"Ring Nebula",
	"Bodes Nebula",
	"Owl Nebula",
];
static NEBULAS_LEN: usize = NEBULAS.len();

static STAR_CLUSTERS: [&'static str; 30] = [
	"Wild Duck",
	"Hyades",
	"Coma",
	"Butterfly",
	"Messier 7",
	"Pleiades",
	"Beehive Cluster",
	"Pearl Cluster",
	"Hodge 301",
	"Jewel Box Cluster",
	"Wishing Well Cluster",
	"Diamond Cluster",
	"Trumpler 10",
	"Collinder 140",
	"Liller 1",
	"Koposov II",
	"Koposov I",
	"Djorgovski 1",
	"Arp-Madore 1",
	"NGC 6144",
	"NGC 2808",
	"NGC 1783",
	"Messier 107",
	"Messier 70",
	"Omega Centauri",
	"Palomar 12",
	"Palomar 4",
	"Palomar 6",
	"Pyxis Cluster",
	"Segue 3",
];
static STAR_CLUSTERS_LEN: usize = STAR_CLUSTERS.len();

static CONSTELLATIONS: [&'static str; 21] = [
	"Big Dipper",
	"Litte Dipper",
	"Orion",
	"Leo",
	"Gemini",
	"Cancer",
	"Canis Minor",
	"Canis Major",
	"Ursa Major",
	"Ursa Minor",
	"Virgo",
	"Libra",
	"Scorpius",
	"Sagittarius",
	"Lyra",
	"Capricornus",
	"Aquarius",
	"Pisces",
	"Aries",
	"Leo Minor",
	"Auriga",
];
static CONSTELLATIONS_LEN: usize = CONSTELLATIONS.len();

static STARS: [&'static str; 19] = [
	"Sun",
	"Proxima Centauri",
	"Rigil Kentaurus",
	"Barnards Star",
	"Wolf 359",
	"Luyten 726-8A",
	"Luyten 726-8B",
	"Sirius A",
	"Sirius B",
	"Ross 154",
	"Ross 248",
	"Procyon A",
	"Procyon B",
	"Vega",
	"Rigel",
	"Arcturus",
	"Betelgeuse",
	"Mahasim",
	"Polaris",
];
static STARS_LEN: usize = STARS.len();

static AGENCYS: [&'static str; 14] = [
	"National Aeronautics and Space Administration",
	"European Space Agency",
	"German Aerospace Center",
	"Indian Space Research Organization",
	"China National Space Administration",
	"UK Space Agency",
	"Brazilian Space Agency",
	"Mexican Space Agency",
	"Israeli Space Agency",
	"Italian Space Agency",
	"Japan Aerospace Exploration Agency",
	"National Space Agency of Ukraine",
	"Russian Federal Space Agency",
	"Swedish National Space Board",
];
static AGENCYS_LEN: usize = AGENCYS.len();

static AGENCYS_ABV: [&'static str; 15] = [
	"NASA",
	"AEM",
	"AEB",
	"UKSA",
	"CSA",
	"CNSA",
	"ESA",
	"DLR",
	"ISRO",
	"JAXA",
	"ISA",
	"CNES",
	"NSAU",
	"ROSCOSMOS",
	"SNSB",
];
static AGENCYS_ABV_LEN: usize = AGENCYS_ABV.len();

static NASA_SPACE_CRAFTS: [&'static str; 10] = [
	"Orion",
	"Mercury",
	"Gemini",
	"Apollo",
	"Enterprise",
	"Columbia",
	"Challenger",
	"Discovery",
	"Atlantis",
	"Endeavour",
];
static NASA_SPACE_CRAFTS_LEN: usize = NASA_SPACE_CRAFTS.len();

static COMPANYS: [&'static str; 9] = [
	"Rocket Lab",
	"SpaceX",
	"Blue Origin",
	"Virgin Galactic",
	"SpaceDev",
	"Bigelow Aerospace",
	"Orbital Sciences",
	"JPL",
	"NASA Jet Propulsion Laboratory",
];
static COMPANYS_LEN: usize = COMPANYS.len();

static DISTANCE_MEASUREMENTS: [&'static str; 5] = [
	"light years",
	"AU",
	"parsecs",
	"kiloparsecs",
	"megaparsecs",
];
static DISTANCE_MEASUREMENTS_LEN: usize = DISTANCE_MEASUREMENTS.len();

static METEORITES: [&'static str; 162] = [
	"Aarhus",
	"Abee",
	"Adelie Land",
	"Adhi Kot",
	"Adzhi-Bogdo",
	"Santa Rosa de Viterbo",
	"Agen",
	"Akbarpur",
	"Albareto",
	"Allan Hills 84001",
	"Allan Hills A81005",
	"Allegan",
	"Allende",
	"Ambapur Nagla",
	"Andura",
	"Angers",
	"Angra dos Reis",
	"Ankober",
	"Anlong",
	"Annaheim",
	"Appley Bridge",
	"Arbol Solo",
	"Archie",
	"Arroyo Aguiar",
	"Assisi",
	"Atoka",
	"Avanhandava",
	"Bacubirito",
	"Baszkówka",
	"Beardsley",
	"Bellsbank",
	"Bench Crater",
	"Benton",
	"Białystok",
	"Blithfield",
	"Block Island",
	"Bovedy",
	"Brachina",
	"Brahin",
	"Brenham",
	"Buzzard Coulee",
	"Campo del Cielo",
	"Canyon Diablo",
	"Cape York",
	"Carancas",
	"Chambord",
	"Chassigny",
	"Chelyabinsk",
	"Chergach",
	"Chinga",
	"Chinguetti",
	"Claxton",
	"Coahuila",
	"Cranbourne",
	"D'Orbigny",
	"Dronino",
	"Eagle Station",
	"Elbogen",
	"Ensisheim",
	"Esquel",
	"Fukang",
	"Gancedo",
	"Gao–Guenie",
	"Gay Gulch",
	"Gebel Kamil",
	"Gibeon",
	"Goose Lake",
	"Grant",
	"Hadley Rille",
	"Heat Shield Rock",
	"Hoba",
	"Homestead",
	"Hraschina",
	"Huckitta",
	"Imilac",
	"Itqiy",
	"Kaidun",
	"Kainsaz",
	"Karoonda",
	"Kesen",
	"Krasnojarsk",
	"L'Aigle",
	"Lac Dodon",
	"Lake Murray",
	"Loreto",
	"Los Angeles",
	"Łowicz",
	"Mackinac Island",
	"Mbozi",
	"Middlesbrough",
	"Millbillillie",
	"Mineo",
	"Monte Milone",
	"Moss",
	"Mundrabilla",
	"Muonionalusta",
	"Murchison",
	"Nakhla",
	"Nantan",
	"Neuschwanstein",
	"Northwest Africa 7034",
	"Northwest Africa 7325",
	"Norton County",
	"Novato",
	"Northwest Africa 3009",
	"Oileán Ruaidh (Martian)",
	"Old Woman",
	"Oldenburg",
	"Omolon",
	"Orgueil",
	"Ornans",
	"Osseo",
	"Österplana 065",
	"Ourique",
	"Pallasovka",
	"Paragould",
	"Park Forest",
	"Pavlovka",
	"Peace River",
	"Peekskill",
	"Penouille",
	"Polonnaruwa",
	"High Possil",
	"Příbram",
	"Pultusk",
	"Qidong",
	"Richardton",
	"Santa Vitoria do Palmar",
	"Sayh al Uhaymir 169",
	"Seymchan",
	"Shelter Island",
	"Shergotty",
	"Sikhote-Alin",
	"Sołtmany",
	"Springwater",
	"St-Robert",
	"Stannern",
	"Sulagiri",
	"Sutter's Mill",
	"Sylacauga",
	"Tagish Lake",
	"Tamdakht",
	"Tenham",
	"Texas Fireball",
	"Tissint",
	"Tlacotepec",
	"Toluca",
	"Treysa",
	"Twannberg",
	"Veliky Ustyug",
	"Vermillion",
	"Weston",
	"Willamette",
	"Winona",
	"Wold Cottage",
	"Yamato 000593",
	"Yamato 691",
	"Yamato 791197",
	"Yardymly",
	"Zagami",
	"Zaisho",
	"Zaklodzie",
];
static METEORITES_LEN: usize = METEORITES.len();

static LAUNCH_VEHICLES: [&'static str; 28] = [
	"Antares",
	"Ariane 5",
	"Atlas",
	"Diamant",
	"Dnepr",
	"Delta",
	"Electron",
	"Energia",
	"Europa",
	"Falcon 9",
	"Falcon Heavy",
	"GSLV",
	"Juno",
	"Long March",
	"Mercury-Redstone",
	"Minotaur",
	"Pegasus",
	"Proton",
	"PSLV",
	"Safir",
	"Shavit",
	"Saturn IV",
	"Semiorka",
	"Soyouz",
	"Titan",
	"Vega",
	"Veronique",
	"Zenit",
];
static LAUNCH_VEHICLES_LEN: usize = LAUNCH_VEHICLES.len();
