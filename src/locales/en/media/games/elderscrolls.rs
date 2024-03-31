use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn es_characters() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn es_games() -> String {
	GAMES[seeder::gen_range(0..GAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn es_location() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn es_factions() -> String {
	FACTIONS[seeder::gen_range(0..FACTIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn es_events() -> String {
	EVENTS[seeder::gen_range(0..EVENTS_LEN)].to_string()
}

static CHARACTERS: [&'static str; 700] = [
    "ATor",
    "Abnur Tharn",
    "Acilius Bolar",
    "AdaSoom Dir-Kamal",
    "Adrimk",
    "Aethra",
    "Agnes of Glenmoril",
    "Agnorith Septim",
    "Ahnurr",
    "Ahrtabazus",
    "Aiden Direnni",
    "Akatosh",
    "Akel",
    "Akha",
    "Alandro Sul",
    "Albrecht Theophannes Bombidius",
    "Alduin",
    "Alessia",
    "Aliera",
    "Alisanne Dupre",
    "Alkhan",
    "Alkosh",
    "Alldimar the Ghostmaker",
    "Allena Benoch",
    "Almalexia",
    "Ami-El",
    "Amiel Septim",
    "Amodetha",
    "Ancano",
    "Andorak Septim",
    "Anequina Sharp-Tongue",
    "Antiochus Septim",
    "Antoine Dubois",
    "Anton Deleyn I",
    "Anton Deleyn II",
    "Antus Pinder",
    "Anu",
    "Anui-El",
    "Aralor",
    "Arden Sul",
    "Ariella Septim",
    "Arius",
    "Arkan",
    "Arkay",
    "Arlimahera",
    "Arn",
    "Arrovan",
    "Arslan II",
    "Arthago",
    "Asgeir",
    "Ashkhan",
    "Askar",
    "Asliel Direnni",
    "Asurn Ice-Breaker",
    "Atak",
    "Atakota",
    "Athras",
    "Athyn Llethan",
    "Attrebus Mede",
    "Auberon Flyte",
    "Augurius Bucco",
    "Auri-El",
    "Avalea",
    "Ayaan-si",
    "Ayalea",
    "Ayrenn",
    "Azura",
    "Baan Dar",
    "Barbas",
    "Barilzar",
    "Baron Volag",
    "Bashomon",
    "Basil (Character)",
    "Beech (Character)",
    "Beela-Eeto",
    "Beela-Kaar",
    "Belharza",
    "Bendu Olo",
    "Beredalmo the Signifier",
    "Bergamot Deleyn",
    "Berit",
    "Bernadette Bantien",
    "Boethiah",
    "Borgas",
    "Bosriel",
    "Boziikkodstrun",
    "Braeloque",
    "Brazollus Dor",
    "Breff the Elder",
    "Brendan the Persistent",
    "Brindisi Dorom",
    "Britte (Lore)",
    "Brunl",
    "Brusef Amelion",
    "Brynjolf",
    "Bthuand Mzahnch",
    "Caddach",
    "Cade",
    "Caius Cosades",
    "Calaxes Septim",
    "Callisos",
    "Camoran Anaxemes",
    "Camoran Kaltos",
    "Camoran Usurper",
    "Captain Jytte",
    "Captain Wereshark",
    "Carolyna",
    "Casimir II",
    "Cassynder Septim",
    "Cassyr Whitley",
    "Castus Philidus",
    "Catchica",
    "Caula Voria",
    "Celethelel the Singer",
    "Cephorus Septim",
    "Cephorus Septim II",
    "Chaero Gemullus",
    "Champion of the Mages Guild",
    "Charwich (Character)",
    "Cirroc the Lofty",
    "Clavicus Vile",
    "Clavilla",
    "Clyton J. Wifflington",
    "Conoon Chodala",
    "Corda",
    "Count",
    "Cuhlecain",
    "Dagoth Acra",
    "Dagoth Tython",
    "Dagoth Ur (Character)",
    "Darius Shano",
    "Darloc Brae",
    "Daughters of Coldharbour",
    "Delphine",
    "Delyn",
    "Derek the Tall",
    "Devir-Mir",
    "Diagna",
    "Dibella",
    "Dibella",
    "Dinieras-Ves",
    "Divad Hunding",
    "Divayth Fyr",
    "Donella",
    "Dorald Larich",
    "Dralsi Indoril",
    "Drawing-Flame",
    "Drayven Indoril",
    "Drengr Bronze-Helm",
    "DroZel",
    "Druids",
    "Druids of Galen",
    "Dumac",
    "Durcorach the Black Drake",
    "Eadwyre",
    "Eamond Guimard",
    "Ebel Septim",
    "Edward",
    "Ehlnofey",
    "Elgryr the Unminded",
    "Eloisa Septim",
    "Elomion",
    "Emer Dareloth",
    "Emeric of Cumberland",
    "Empress Tavia",
    "Enman Septim",
    "Ephen",
    "Ephrem Benirus",
    "Eplear",
    "Eric of Guis",
    "Erling",
    "Eshita",
    "Eternal Champion",
    "Ettiene Volusus",
    "Ezhmaar Sul",
    "Fa-Nuit-Hen",
    "Fadomai",
    "Failed Incarnate",
    "Faire Agarwen",
    "Far-Sighted Uche",
    "Farangel Gardner",
    "Faume Toad-Eye",
    "Felms",
    "Fervidius Tharn",
    "Finna (Lore)",
    "Fjokki the Bard",
    "Fjotli Cruel-Sea",
    "Flesus Tijjo",
    "Flinthild Demon-Hunter",
    "Folbert the Wide",
    "Frandar Hunding",
    "Freida Oaken-Wand",
    "Freydis",
    "Friar Lylim",
    "Friga Shatter-Shield",
    "Froa",
    "Fugitive (Blades)",
    "Furl-Of-Fresh-Leaves",
    "Gaiden Shinji",
    "Galana Septim",
    "Gallivere Lariat",
    "Garnag",
    "Garridan Stalrous",
    "Gaspard Esmery",
    "Geirmund",
    "Geldall Septim",
    "Gellir",
    "General Decianus",
    "General Jonna",
    "General Takar",
    "General Tullius",
    "Geocrates Varnus",
    "Gharakul (Orsinium)",
    "Gharen",
    "Ghelin-Brol",
    "Gjalund",
    "Glenhwyfaunva",
    "Golkarr",
    "Golthog",
    "Gordhaur the Shaper",
    "Gorieus",
    "Gortwog gro-Nagorm",
    "Gothlyr",
    "Graddock",
    "Gratian Caerellius",
    "Greyf",
    "Grimkell",
    "Grosta (Lore)",
    "Gudrun",
    "Gunnar",
    "Gysilla",
    "Hadhuul",
    "Hakkvild Yashnag-Slayer",
    "Hallgerd",
    "Hana",
    "Hanse",
    "Harald",
    "Haromir of Copper and Tea",
    "Hasedoki",
    "Hazadir",
    "Hean",
    "Heimdall the Frenzied",
    "Helgreir Lute-Voice",
    "Hellena",
    "Helseth",
    "Henantier the Outsider",
    "Herbane",
    "Hermaeus Mora",
    "Hermeskr",
    "Hero of Dawnstar",
    "Hero of Kvatch",
    "Hestra",
    "Hidellith",
    "High Priest Uluscant",
    "Hira",
    "Hircine",
    "Hjalmer",
    "Hoag Merkiller",
    "Hoag Stormcloak",
    "HoonDing",
    "Horavatha",
    "Hosgunn Crossed-Daggers",
    "Hranvard Frostfinger",
    "Hroki (Whiterun)",
    "Hrolfdir",
    "Hrothmund the Red",
    "Huna",
    "Iric Harad Egun",
    "Iachesis",
    "Ianus Faleria",
    "Ideal Masters",
    "Ilzheven",
    "Imbrallius",
    "Indoril Brindisi Dorom",
    "Indoril Nerevar",
    "Inga",
    "Ingjaldr White-Eye",
    "Investigator Vale",
    "Inzolicus",
    "Ioa",
    "Irenbis Songblade",
    "Irthvyd the Impassive",
    "Isabella",
    "Istlod",
    "Ithelia",
    "Ius",
    "Ja-Khajay",
    "Jagar Tharn",
    "Jaiv-Yora",
    "Jastyaga",
    "Jeek of the River",
    "Jhunal",
    "Jiub",
    "Jode",
    "Jofrior",
    "Jolethe Septim",
    "Jonder the Tiny",
    "Jone",
    "Jonni",
    "Jorg Helmbolg",
    "Jovron Direnni",
    "Jsashe",
    "Jubal-lun-Sul",
    "Juilek Cyrodiil",
    "Julianos",
    "Justin",
    "Justinius Poluhnius",
    "Jyggalag",
    "Kagrenac",
    "Kaladas",
    "Kalien",
    "Kantus Jeril",
    "Kastav",
    "Katariah",
    "Katisha",
    "Keirgo",
    "Kena Warfel Tomasin",
    "Khartag",
    "Khasha",
    "Khenarthi",
    "Khunzar-ri",
    "King Anumaril",
    "King Aphren",
    "King Barynia",
    "King Hurlburt",
    "King Lleromo",
    "Kintyra Septim",
    "Kintyra Septim II",
    "Kjoric the White",
    "Kluwe",
    "Koniinge",
    "Kota",
    "Kynareth",
    "Kyne",
    "Kyrnil Long-Nose",
    "Lady Arannelya",
    "Lamae Bal",
    "Last Dragonborn",
    "Legate Cassia",
    "Legate Rikke",
    "Leki",
    "Lieutenant Lein-Barduik",
    "Llothis",
    "Logrolf",
    "Lord Bowyn",
    "Lord Janic",
    "Lord Naarifin",
    "Lord Nunex Faleria",
    "Lord Thone",
    "Lord Vanech",
    "Lord Vhokken",
    "Lord Xellicles Pinos-Revina",
    "Loreth",
    "Lorkh",
    "Lorkhaj",
    "Lorkhan",
    "Lucien Lachance",
    "Lucina Faleria",
    "Lusty Argonian Maid",
    "Lydette Viliane",
    "Lymdrenn Telvanni",
    "Lyrisius",
    "Maiq the Liar",
    "Mabjaarn Flame-Hair",
    "Macalla",
    "Macke of the Piercing Eyes",
    "Mackkan",
    "Magna",
    "Magnus",
    "Magnus Septim",
    "Magrus",
    "Malacath",
    "Malooc",
    "Malur Omayn",
    "Mane",
    "Mannimarco",
    "Mansel Sesnit",
    "Mantiarco",
    "Manwe",
    "Mara",
    "Martin Septim",
    "Marukh",
    "Masser",
    "Master Tunnel Rat",
    "Mathieu Bellamonts Mother",
    "Mats",
    "Mauloch",
    "Maxivian Faleria",
    "Mehra Nabisi",
    "Mehrunes Dagon",
    "Meksim the Walker",
    "Menro",
    "Mentin",
    "Mephala",
    "Meris",
    "Merkyllian Ramth",
    "Mhorus",
    "Mhuvnak",
    "Mirtil Angoth",
    "Mith",
    "Mithas",
    "Mnemoli",
    "Modellus",
    "Molag Bal",
    "Montocai",
    "Morachellis",
    "Moraelyn",
    "Morgiah",
    "Morian Zenas",
    "Moricar",
    "Morihatha Septim",
    "Morihaus",
    "Mortyn",
    "Morwha",
    "Moth Priest",
    "Mother Pascost",
    "Mryfwiil the Withdrawn",
    "Muzariah",
    "Mynisera",
    "Mzunchend",
    "Naenra Waerr",
    "Nafaalilargus",
    "Nai",
    "Nai Tyrol-Llar",
    "Namira",
    "Namira",
    "Narimaya",
    "Naryu Virian",
    "Nataly Dravarol",
    "Nataly Dravarol",
    "Nchunak",
    "Neloth",
    "Neransi Faleria",
    "Nerevarine",
    "Neria Relethyl",
    "Nhad-hatta",
    "Nienolas Ulwarth",
    "Night Mother",
    "Nilara Ienth",
    "Nir",
    "Nocturnal",
    "Nomu",
    "Nulfaga",
    "Nurarion the Perfect",
    "Obeth Arnesian",
    "Odahviing",
    "Oghma",
    "Oitos",
    "Olms",
    "Onsi",
    "Oresme",
    "Oreyn Bearclaw",
    "Orgnum",
    "Orkey",
    "Orthor",
    "Othrok",
    "Paarthurnax",
    "Padomay",
    "Pelagius Septim",
    "Pelagius Septim II",
    "Pelagius Septim III",
    "Pelagius Septim III",
    "Pelagius Septim IV",
    "Pelagius Septim IV",
    "Pelinal Whitestrake",
    "Pelladil Direnni",
    "Per Vetersen",
    "Peryite",
    "Phynaster",
    "Pierric",
    "Plontinu",
    "Porcia Loran",
    "Potema Septim",
    "Prelates",
    "Prince Bathorgh",
    "Princess Rakma",
    "Princess Talara",
    "Prior Emelene Madrine",
    "Purilla Falen",
    "Pyokos",
    "Queen Bianki",
    "Queen Lian",
    "Quintilla",
    "Rleyt-harhr",
    "Ravindra",
    "Rajhin",
    "Ramth the Greater",
    "Randagulf",
    "Randic Torn",
    "Rangidil Ketil",
    "Rasha (Lore)",
    "Ravate",
    "Raven Direnni",
    "Ravila Neryon",
    "Rayelle",
    "Red Bramman",
    "Reglius",
    "Regulus Sardecus",
    "Reman Cyrodiil",
    "Reman Cyrodiil II",
    "Reman Cyrodiil III",
    "Reman Karoodil",
    "Rhorlak (Lore)",
    "Ria Silmane",
    "Rid-Thar-riDatta",
    "RiddleThar",
    "Right-Foot-Rock",
    "Rilms",
    "Roeth",
    "Roleke",
    "Roris",
    "Ru-Ri",
    "Ruptga",
    "Ryain Direnni",
    "Rynandor",
    "Srendarr",
    "Saeel",
    "Sai",
    "Saint Kaladas",
    "Saint Vorys",
    "Sanguine",
    "Sanit-Kil",
    "Saradin the Lost Love",
    "Sargenius",
    "Satak",
    "Satakal",
    "Savirien Chorak",
    "Secunda",
    "Senecus Goddkey",
    "Sep",
    "Seryn",
    "Sethiete",
    "Shadar Erabenimsun",
    "Shagrath",
    "Shalidor",
    "Shehs",
    "Shelly",
    "Sheogorath",
    "Sheor",
    "Shezarr",
    "Shezarrine",
    "Shor",
    "Shor",
    "Shor-El",
    "Shrike",
    "Sil Rothril",
    "Silarian",
    "Silk",
    "Sir Amiel Lannus",
    "Sir Berich Vlindrel",
    "Sir Byric of the Flame",
    "Sir Caius",
    "Sir Casimir",
    "Sir Gregory (Knights of the Nine)",
    "Sir Henrik",
    "Sir Juncan",
    "Sir Ralvas",
    "Sir Roderic",
    "Sir Torolf",
    "Sithis",
    "Skalg",
    "Skardan Free Winter",
    "Skegglund Stormcloak",
    "Skorm Snow-Strider",
    "Snow Prince",
    "Sotha Sil",
    "Soul of Conflict",
    "Springheel Jak",
    "Ssaass",
    "Stendarr",
    "Straw (Lore)",
    "Strom the White",
    "Stuhn",
    "Stuhn",
    "Styriche",
    "Sun-in-Shadow",
    "Suturah",
    "Svargrim",
    "Svartr",
    "Sven Advensen",
    "Symmachus",
    "Syrabane",
    "Taig Hoïnart",
    "Talen Vandas",
    "Talos",
    "Talos",
    "Tamrialle",
    "Tava",
    "Temylda",
    "Teo Bravillius Tasus",
    "Terr",
    "Terrfyg",
    "Thagore",
    "Thalthil Dres",
    "Thassad II",
    "The Agent",
    "The Forgotten Hero",
    "The Hoar Father",
    "The Silvenar",
    "The Underking",
    "Therion",
    "Therris",
    "Thian",
    "Thitsar-je",
    "Thonica",
    "Thoricles Romus",
    "Thules the Gibbering",
    "Thulgeg",
    "Tiber Septim",
    "Timmy",
    "Tissina Gray",
    "Titus Alorius",
    "Titus Alorius",
    "Titus Mede I",
    "Titus Mede II",
    "Titus Mede II",
    "Tjurhane Fyrre",
    "Tongues",
    "Topal the Pilot",
    "Tor",
    "Torug gro-Igron",
    "Tosh Raka",
    "Trinimac",
    "Tsleeixth",
    "Tuwhacca",
    "Tuldinwae",
    "Tulvar the Unmentioned",
    "Tunaska",
    "Turala",
    "Tyr",
    "Tysnal",
    "Ulaqth",
    "Ulfe Gersen",
    "Ulfgi Anvil-Hand",
    "Ulfric Stormcloak",
    "Ulliceta gra-Kogg",
    "Ulvul Llaren",
    "Umaril",
    "Uriel Septim I",
    "Uriel Septim II",
    "Uriel Septim III",
    "Uriel Septim III",
    "Uriel Septim IV",
    "Uriel Septim V",
    "Uriel Septim VI",
    "Uriel Septim VII",
    "Uriel Septim VII",
    "Urken",
    "Urlach",
    "Utheilla Direnni",
    "Valdimar (Lore)",
    "Valin Ulen",
    "Vanech",
    "Vanus Galerion",
    "Varbarenth",
    "Varen Aquilarios",
    "Vashu-Pir",
    "Vasi Hadrach",
    "Veloth",
    "Veraxia Tharn",
    "Vernaccus",
    "Versidue Shaie",
    "Vestige",
    "Viana the Pure",
    "Vilur Ulen",
    "Virmaril the Betrayer",
    "Vithrelnaak",
    "Vitraen Vitharn",
    "Vivec",
    "Voernet",
    "Vondham Barres",
    "Vorian Direnni",
    "Voth Karlyss",
    "Vrage",
    "Vulstaed",
    "Vune",
    "Vust the Smiler",
    "Warday Akor",
    "Warlord Attrebus",
    "Waughin Jarth",
    "Willow",
    "Wulfharth",
    "Xarxes",
    "Xen",
    "Xero-Lyg",
    "Xhon-Mehl",
    "Yffre",
    "Yaghoub",
    "Yashnag gro-Yazgu",
    "Ylgar",
    "Yngol",
    "Ysgramor",
    "Ysmir",
    "Zen",
    "Zeht",
    "Zenithar",
    "Zizzeen",
    "Zuathas the Clever-Cutting Man",
    "Zurin Arctus",
    "Zuuk (Character)",
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static EVENTS: [&'static str; 167] = [
    "Aetherium Wars",
    "Akaviri Invasions",
    "Alchemical Symposium",
    "Alessian Slave Rebellion",
    "Alliance War",
    "An-Xileel Daedric resistance",
    "Anequine Conquests",
    "Arnesian War",
    "Battle atop Mount Anthor",
    "Battle for Whiterun",
    "Battle of Ald Marak",
    "Battle of Argonia",
    "Battle of Bjoulsae",
    "Battle of Bodrum",
    "Battle of Chalman Keep",
    "Battle of Cryngaine Field",
    "Battle of Dragontooth",
    "Battle of Falconstar",
    "Battle of Falinesti",
    "Battle of Firewaves",
    "Battle of Glenumbra Moors",
    "Battle of Hattu Mountain",
    "Battle of Hunding Bay",
    "Battle of Ichidag",
    "Battle of Ionith",
    "Battle of Kvatch",
    "Battle of Markwasten Moor",
    "Battle of Molag Beran",
    "Battle of Old Hroldan",
    "Battle of Pale Pass",
    "Battle of Red Mountain",
    "Battle of Rourken-Shalidor",
    "Battle of Sancre Tor",
    "Battle of Sungard",
    "Battle of the Moesring",
    "Battle of the Red Ring",
    "Battle of Wightmoor",
    "Battle of Zelinin",
    "Blackwater War",
    "Cleansing of the High Fane",
    "Coldharbour Compact",
    "Convention",
    "Council of Bardmont",
    "Covenant",
    "Dawn Era",
    "Day of Final Passage",
    "Disappearance of the Dwarves",
    "Disaster at Ionith",
    "Dragon Break",
    "Dragon Crisis",
    "Dragon War",
    "Era",
    "Fall of Aldruhn",
    "Fall of White-Gold Tower",
    "Feast of the Dead",
    "Feast of the Tiger",
    "Fifth Era",
    "First Battle of Iones Oblivion Gate",
    "First Era",
    "First Pogrom",
    "Firsthold Revolt",
    "Five Year War",
    "Flower Day",
    "Flower Festival",
    "Forebear Truce",
    "Forsworn Uprising",
    "Four Score War",
    "Fourth Era",
    "Frostfall Coup",
    "Gauntlet (Holiday)",
    "Great Burn",
    "Great Moot",
    "Great War",
    "Greymarch",
    "Hammerfell Civil War",
    "Harrowstorms",
    "Hel Anseilak",
    "Histories of Cyrodiil",
    "Imperial Simulacrum",
    "Interregnum",
    "Kagrumez Trials",
    "Karvinasim",
    "King Ransers War",
    "Knahaten Flu",
    "Kvatch Bread Riots",
    "Landfall",
    "Levitation Act",
    "Llodos Plague",
    "Mages War",
    "March of Beauty",
    "March of Thirst",
    "Markarth Incident",
    "Marukhs Day",
    "Merethic Era",
    "Moot",
    "Narfinsel Schism",
    "Necromancy Ban of 3E 431",
    "Night of Green Fire",
    "Night of Tears",
    "Ninth Era",
    "Nord-Breton War",
    "Nordic-Falmer War",
    "North Winds Prayer",
    "Oblivion Crisis",
    "Oblivion Portals",
    "P",
    "Pact of Chieftains",
    "Planemeld",
    "Red Year",
    "Red Year",
    "Rise of Daggerfall",
    "Rite of Naming",
    "Ritual of the Innocent Quarry",
    "Sack of Mournhold",
    "Sack of Skywatch",
    "Sack of Winterhold",
    "Sacking of Black Gate",
    "Scouring of Wendelbek",
    "Second Battle of Hunding Bay",
    "Second Era",
    "Second Treaty of Stros Mkai",
    "Shade of the Revenant",
    "Siege of Abernanit",
    "Siege of Orsinium",
    "Siege of Red Eagles Stronghold",
    "Siege of Sky Haven Temple",
    "Siege of Solitude",
    "Siege of the Imperial City (4E 49)",
    "Siege of Wayrest",
    "Skyrim Civil War",
    "Skyrim Conquests",
    "Slaughter of Torval",
    "Soulburst",
    "Storm of Separation",
    "Stormcrown Interregnum",
    "Suns Death",
    "The Great Battle",
    "The Great Collapse",
    "The Return",
    "Third Era",
    "Thrassian Plague",
    "Tiber Wars",
    "Time Wound",
    "Timeline",
    "Torns Sword-Hunt",
    "Tournament of Stamina",
    "Traditional Nordic Duel",
    "Treaty of Gradkeep",
    "Treaty of Stros Mkai",
    "Treaty of the Armistice",
    "Umbriel Crisis",
    "Void Nights",
    "War of Bendr-Mahk",
    "War of Betony",
    "War of Manifest Metaphors",
    "War of Righteousness",
    "War of Succession",
    "War of the Blue Divide",
    "War of the Crag",
    "War of the First Council",
    "War of the Isle",
    "War of the Red Diamond",
    "War of the Uvichil",
    "Warp in the West",
    "Warrior Wave",
    "Wild Hunt",
    "Winterhold Rebellion",
];
static EVENTS_LEN: usize = EVENTS.len();

static FACTIONS: [&'static str; 177] = [
    "Afterdark Society",
    "Ahemmusa Tribe",
    "Aldmeri Dominion",
    "Alessian Empire",
    "Alessian Order",
    "All Flags Navy",
    "An-Xileel",
    "Ansei",
    "Ashabah",
    "Ashlanders",
    "Atrius Building Commission",
    "Baandari Clan",
    "Bal Molagmer",
    "Barsaebic Ayleids",
    "Bathogorgen",
    "Black Hand",
    "Category:Black Hand Members",
    "Black-Briars",
    "Blacksap Rebellion",
    "Blades",
    "Bloodskal Clan",
    "Bonsamu",
    "Camonna Tong",
    "Chantry Paladin Order",
    "Circle of Captains",
    "Colovian Estates",
    "Conclave of the Eleven Forces",
    "Council of Bardmont",
    "Crow-Wife Clan",
    "Crusaders",
    "Cult of Emperor Zero",
    "Cult of the Ancestor Moth",
    "Cyrodiil Vampyrum Order",
    "Daggerfall Covenant",
    "Dakarn",
    "Dark Brotherhood",
    "Dervishes of Rihad",
    "Diodata",
    "Direfrost Family",
    "Direnni Dynasty",
    "Direnni Hegemony",
    "Dominion Military",
    "Dominion Navy",
    "Dragon Cult",
    "Dragonguard",
    "Druids",
    "Druids of Galen",
    "Dubois and Sons Carpentry",
    "Ebonheart Pact",
    "Eight Traitors",
    "Eighteenth Legion",
    "Eighth Legion",
    "Elsweyr Confederacy",
    "End of Times Cult",
    "Erabenimsun Tribe",
    "Euraxian Legion",
    "Expeditionary Force",
    "Factions",
    "Fifteenth Legion",
    "Fifth Legion",
    "Fighters Guild",
    "First Council",
    "Five Hundred Companions",
    "Forsworn",
    "Forsworn Rebellion",
    "Fourteenth Legion",
    "Fourth Legion",
    "Gaspards Stalkers",
    "Glenmoril Witches",
    "Great Houses",
    "Great Moot",
    "Haarvenu",
    "Hands of Almalexia (Morrowind)",
    "Hiradirge",
    "Horme",
    "House Dagoth",
    "House Dres",
    "House Hlaalu",
    "House Indoril",
    "House Raathim",
    "House Redoran",
    "House Sadras",
    "House Sathil",
    "House Suda",
    "House Telvanni",
    "Category:House Telvanni Members",
    "House Tharn",
    "Imperial Cult",
    "Imperial Geographic Society",
    "Imperial Guard Division",
    "Imperial Legion",
    "Imperial Navy",
    "Imperial University",
    "Keepers of the Razor",
    "Keerilth Clan",
    "Knights of Saint Pelin",
    "Knights of the Dragon",
    "Knights of the Flame",
    "Knights of the Nine (Faction)",
    "Knights of the True Horn",
    "Korgari",
    "Kragen Clan",
    "Lainlyn Family",
    "Legion of Chorrol",
    "Lyrezi Clan",
    "Mages Guild",
    "Mananauts",
    "Markyn",
    "Marukhati Selective",
    "Moraelyns Companions",
    "Morag Tong",
    "Moth Priest",
    "Mythic Dawn",
    "Na-Totambu",
    "Nerevarine Cult",
    "New Temple",
    "New West Navy",
    "Nightingales",
    "Nine Divines (Faction)",
    "Ninth Legion",
    "Nordic Empire",
    "Office of Introductory Studies",
    "Order of Diagna",
    "Order of Doctrine and Ordination",
    "Order of Inquisition",
    "Order of the Black Worm",
    "Order of the Candle",
    "Order of the Hour",
    "Order of the Lamp",
    "Order of the Raven",
    "Order of the Waking Flame",
    "Order of the Watch",
    "Order of War",
    "Pack of Bards",
    "Parikh Tribe",
    "Penitus Oculatus",
    "Psijic Order",
    "Red Sabre",
    "Renrijra Krin",
    "Restless League",
    "Rourken Clan",
    "Sapiarchs",
    "Second Aldmeri Dominion",
    "Second Empire",
    "Second Legion",
    "Sect of Harmonious Masters",
    "Selenu Clan",
    "Seventeenth Legion",
    "Seventh Legion",
    "Shadow Legion",
    "Shadowscales",
    "Shellbacks",
    "Silver Crescents",
    "Silver-Bloods",
    "Sixth Legion",
    "Skaal",
    "Snow-Shod",
    "Stormcloak Clan",
    "Sun Birds",
    "Sword-Singers",
    "Telboth",
    "Temple of Sethiete",
    "Tenth Legion",
    "Thalmor",
    "Thazahrr Cartel",
    "The Beautiful",
    "The Claws",
    "The College of Whispers",
    "The Companions",
    "The Crimson Dirks",
    "The Crowns",
    "The Eclipse",
    "The Lhotunics",
    "The Natives",
    "Thieves Guild",
    "Third Legion",
    "Thousand-Strong of Sedor",
];
static FACTIONS_LEN: usize = FACTIONS.len();

static GAMES: [&'static str; 9] = [
	"The Elder Scrolls Arena",
	"The Elder Scrolls II: Daggerfal",
	"The Elder Scrolls III: Morrowind",
	"The Elder Scrolls IV: Oblivion",
	"The Elder Scrolls V: Skyrim",
	"The Elder Scrolls Online",
	"The Elder Scrolls: Blades",
	"The Elder Scrolls Legend: Battlespire",
	"The Elder Scrolls Adventures: Redguard",
];
static GAMES_LEN: usize = GAMES.len();

static LOCATIONS: [&'static str; 378] = [
    "Aalto",
    "Abecean Coast",
    "Aetherius",
    "Akavir",
    "Akos Kasaz",
    "Alcaire",
    "Ald Iuval",
    "Ald Olyra",
    "Ald Sotha",
    "Ald Umbeil",
    "Aldruhn",
    "Aldmeris",
    "Alduins Wall",
    "Alikr Desert",
    "Alinor",
    "Anticlere",
    "Anvil",
    "Anvil Castle (Oblivion)",
    "Apocrypha",
    "Archons Grove",
    "Arenthia",
    "Arnesia",
    "Artaeum",
    "Ascadian Isles",
    "Ashen Forge",
    "Ashlands",
    "Ashurnibibi",
    "Atmora",
    "Aurbis",
    "Avanchnzel",
    "Azuras Coast",
    "Baar Dau",
    "Bal Tereg",
    "Baldaras",
    "Balmora",
    "Bardmont",
    "Belkarth",
    "Bergama",
    "Bitter Coast",
    "Bjoulsae River",
    "Black Gate",
    "Black Keirgo",
    "Black Marsh",
    "Black Rocks",
    "Blacklight",
    "Blackrose",
    "Blackrose Prison",
    "Bleak Falls Barrow",
    "Bliss",
    "Bravil",
    "Brindle",
    "Brothers of Strife",
    "Bruma",
    "Caecilly Island",
    "Caer Suvio",
    "Calluis Lar",
    "Camlorn",
    "Cape of the Blue Divide",
    "Castle Bruma",
    "Castle Cheydinhal",
    "Castle Giovesse",
    "Castle Leyawiin",
    "Cathnoquey",
    "Cavern of the Incarnate",
    "Celediil",
    "Census and Excise Office (Slough Point)",
    "Ceporah Tower",
    "Cespar",
    "Ceyatatar",
    "Chalman",
    "Cheydinhal",
    "Chorrol",
    "City Isle",
    "Clockwork City",
    "Cloud Ruler Temple",
    "Cloudrest",
    "College of Aldmeri Propriety",
    "College of Sapiarchs",
    "College of Winterhold",
    "Colored Rooms",
    "Commerce Road",
    "Corgrad Wastes",
    "Corinthe",
    "Corinthe Sanctuary",
    "Cormount",
    "Craghold",
    "Crosswych",
    "Crucible",
    "Cryngaine Field",
    "Crypt of Hearts",
    "Crystal Tower",
    "Cyrodiil",
    "Daenia",
    "Daggerfall",
    "Daggerfall (City)",
    "Darkwater Crossing",
    "Davons Watch",
    "Dawnstar",
    "Deadlands",
    "Dellese Isles",
    "Delodiil",
    "Deshaan",
    "Detritus",
    "Direnni Tower",
    "Diss",
    "Dragon Bridge",
    "Dragons Teeth Mountains",
    "Dragonstar",
    "Drajkmyr Marsh",
    "Duncreigh Bridge",
    "Dune",
    "Dunstad",
    "Dusk",
    "Duskwallow Hamlet",
    "Dwynnen",
    "Eagles Brook",
    "Eastmarch",
    "Ebon Stadmont",
    "Ebonheart",
    "Elden Grove",
    "Elden Root",
    "Elinhir",
    "Elsweyr",
    "Eltheric Ocean",
    "Ephesus",
    "Esroniet",
    "Eton Nir",
    "Evermore",
    "Falinesti",
    "Falkreath",
    "Falkreath Hold",
    "Fang Lair",
    "Fharun",
    "Firemoth",
    "Firewatch",
    "Firsthold",
    "Fort Amol",
    "Fort Ash",
    "Fort Sphinxmoth",
    "Fortress of Ice",
    "Fringe Gyre",
    "Frostcrag Spire",
    "Galen",
    "Garlas Malatar",
    "Gavaudon",
    "Gideon",
    "Gil-Var-Delle",
    "Gilane",
    "Glenpoint",
    "Glenumbra",
    "Glenumbra Moors",
    "Gnisis",
    "Gold Coast",
    "Gorne",
    "Granite Hill",
    "Grazelands",
    "Great Forest",
    "Great Orrery",
    "Great Scathes",
    "Greater Bretony",
    "Green Hall",
    "Greenheart",
    "Greenwall",
    "Greymoor",
    "Gyldenhul Barrow",
    "Hammerfell",
    "Havel Slump",
    "Haven",
    "Heimlyn Keep",
    "Hel Ra Citadel",
    "Hereguard Plantation",
    "Herne (Island)",
    "High Hrothgar",
    "High Rock",
    "Hixinoag",
    "Holamayan Monastery",
    "Horde Mountain",
    "Hsaarik Head",
    "Hubalajads Bay",
    "Ilessan Hills",
    "Imperial City",
    "Inner Sea",
    "Iron Wheel Headquarters",
    "Isle of Balfiera",
    "Isle of Gold",
    "Isle of Summerset",
    "Jehanna",
    "Kamal",
    "Karnwasten",
    "Kavas Rim Pass",
    "Keel-Sakka River",
    "Kemel-Ze",
    "Khartag Point",
    "Khefrem",
    "Kherakah",
    "Kings Guard",
    "Kings Guard Mountains",
    "Kogmenthist Castle",
    "Kozanset",
    "Kragenmoor",
    "Kurallian Mountains",
    "Kvatch",
    "Kynesgrove",
    "Labyrinthian",
    "Lainlyn",
    "Lefthand Empire",
    "Leftunch",
    "Levinace",
    "Leyawiin",
    "Lillandril",
    "Lilmoth",
    "Longhaven",
    "LothNa Caverns",
    "Lyg",
    "Maelstrom of Bal",
    "Marbruk",
    "Markarth",
    "Marog",
    "Menevia",
    "Menevia",
    "Mines of Khuras",
    "Molag Amur",
    "Moonmont",
    "Mor Khazgur",
    "Morrowind",
    "Morthal",
    "Mother Pascosts Tavern",
    "Mountain",
    "Mournhold",
    "Mournoth",
    "Mundus",
    "Murkwood",
    "Murtag Stronghold",
    "Myrkwasa",
    "Narlemae",
    "Narsis",
    "Nchuleft",
    "Necrom",
    "Nerone",
    "Netherworld",
    "Neugrad",
    "New Sheoth",
    "New Sheoth Palace",
    "Nibenay",
    "Nibenay Valley",
    "Nirn",
    "Northpoint",
    "Ogres Tooth Mountains",
    "Old Holds",
    "Old Hroldan",
    "Oldgate",
    "Olenveld",
    "Oliis Bay",
    "Orcrest",
    "Oroy Mansion",
    "Orsinium",
    "Ouada Glaamnir",
    "Pankor",
    "Pells Gate",
    "Pinepeak Cavern",
    "Port Humbled",
    "Port Telvannis",
    "Privateers Hold",
    "Prixon Palace",
    "Pyandonea",
    "Quinrawl Peninsula",
    "Rat and the Pot",
    "Raven Rock",
    "Rawlkha",
    "Red Mountain",
    "Riften",
    "Rimmen",
    "Riverhold",
    "Riverwood",
    "Roscrea",
    "Ruby Throne",
    "Sadrith Mora",
    "Samara",
    "Samuruik",
    "Santaki",
    "Sardavar Leed",
    "Satakalaam",
    "Sathil (Village)",
    "Scathing Bay",
    "Sea of Ghosts",
    "Sea of Pearls",
    "Selenes Web",
    "Senchal",
    "Sentinel",
    "Seyda Neen",
    "Shimmerene",
    "Shivering Isles",
    "Shors Stone",
    "Shornhelm",
    "Sil-Var-Woad",
    "Silvenar (City)",
    "Skavyn",
    "Skaal Village",
    "Skingrad",
    "Skuldafn",
    "Skyreach",
    "Skyrim",
    "Skywatch",
    "Slough Point",
    "Snowhawk",
    "Solitude",
    "Solstheim",
    "Soulrest",
    "Southpoint",
    "Sovngarde (Location)",
    "Spiral Skein",
    "Stonefalls",
    "Stonehills",
    "Storm Talon Temple",
    "Stormhaven",
    "Stormhold",
    "Strid Estuary",
    "Stros MKai",
    "Subo-Lathla",
    "Summerset Isles",
    "Sun-kissed Shores",
    "Sungard",
    "Sunhold",
    "Suran",
    "Sutch",
    "Tamriel",
    "Taneth",
    "Tarlain Heights",
    "Tear",
    "Tel Aedrin",
    "Tel Aruhn",
    "Tel Branora",
    "Tel Mothrivra",
    "Temple of Mara (Umpholo)",
    "Temple of the Ancestor Moths",
    "The Chain",
    "The Dreaming Cave",
    "The Fields of Regret",
    "The Halls of Colossus",
    "The Land Between",
    "The Reach",
    "The Rift",
    "The Systres",
    "Thir River",
    "Thirsk Mead Hall",
    "Thormar",
    "Thorn",
    "Thornmarsh",
    "Throat of the World",
    "Thurzo Fortress",
    "Tigonus",
    "Tomb of Dagoth Morin",
    "Topal Island",
    "Torval",
    "Totambu (Yokuda)",
    "Traitors Tor",
    "Tsaesci",
    "Tyrigel",
    "Ulvor Kus",
    "Umbriel",
    "Umpholo",
    "University of Gwylim",
    "Valenwood",
    "Varanis",
    "Velothi Mountains",
    "Verkarth",
    "Vernim Woods",
    "Vivec City",
    "Volcanoes",
    "Volenfell",
    "Vos",
    "Vounoura",
    "Vulkhel Guard",
    "Vvardenfell",
    "Wasten Coridale",
    "Wayrest",
    "Weir Gate",
    "West Gash",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();
