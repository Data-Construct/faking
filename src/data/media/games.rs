use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn title() -> String {
	let mut rng = rand::thread_rng();
	TITLES[rng.gen_range(0..TITLES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn genre() -> String {
	let mut rng = rand::thread_rng();
	GENRES[rng.gen_range(0..GENRES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn platform() -> String {
	let mut rng = rand::thread_rng();
	PLATFORMS[rng.gen_range(0..PLATFORMS_LEN)].to_string()
}

static TITLES: [&'static str; 228] = [
	"Half-Life",
	"Half-Life: Opposing Force",
	"Half-Life: Blue Shift",
	"Half-Life 2",
	"Half-Life 2: Episode One",
	"Half-Life 2: Episode Two",
	"Half-Life 2: Lost Coast",
	"Portal",
	"Portal 2",
	"Team Fortress Classic",
	"Team Fortress 2",
	"Left 4 Dead",
	"Left 4 Dead 2",
	"Day of Defeat",
	"Ricochet",
	"Dota 2",
	"Counter-Strike",
	"Counter-Strike: Source",
	"Counter-Strike: Global Offensive",
	"Garry's Mod",
	"Super Mario Bros.",
	"Super Mario Bros. 2",
	"Super Mario Bros. 3",
	"Super Mario World",
	"Super Mario Sunshine",
	"Super Mario Galaxy",
	"Super Mario Galaxy 2",
	"Super Mario Odyssey",
	"Mario Kart 64",
	"Mario Kart: Double Dash",
	"Mario Kart DS",
	"Mario Kart Wii",
	"Mario Kart 7",
	"Mario Kart 8",
	"Mario Kart 8 Deluxe",
	"Luigi's Mansion",
	"Animal Crossing",
	"Animal Crossing: Wild World",
	"Animal Crossing: City Folk",
	"Animal Crossing: New Leaf",
	"Civilization III",
	"Civilization IV",
	"Civilization V",
	"Civilization VI",
	"The Legend of Zelda",
	"The Legend of Zelda: Ocarina of Time",
	"The Legend of Zelda: Majora's Mask",
	"The Legend of Zelda: Twilight Princess",
	"The Legend of Zelda: Skyward Sword",
	"The Legend of Zelda: Breath of the Wild",
	"Pong",
	"Pac-Man",
	"Starcraft",
	"Starcraft II",
	"Overwatch",
	"Hearthstone",
	"Halo: Combat Evolved",
	"Halo 2",
	"Halo 3",
	"Halo 3: ODST",
	"Halo: Reach",
	"Halo 4",
	"Halo 5: Guardians",
	"Wii Sports",
	"Wii Sports Resort",
	"Wii Play",
	"Wii Music",
	"Pokémon Red",
	"Pokémon Blue",
	"Pokémon Yellow",
	"Pokémon Gold",
	"Pokémon Silver",
	"Pokémon Crystal",
	"Pokémon Ruby",
	"Pokémon Sapphire",
	"Pokémon FireRed",
	"Pokémon LeafGreen",
	"Pokémon Emerald",
	"Pokémon Diamond",
	"Pokémon Pearl",
	"Pokémon Platinum",
	"Pokémon HeartGold",
	"Pokémon SoulSilver",
	"Pokémon Black",
	"Pokémon White",
	"Pokémon Black 2",
	"Pokémon White 2",
	"Pokémon X",
	"Pokémon Y",
	"Pokémon Omega Ruby",
	"Pokémon Alpha Sapphire",
	"Pokémon Sun",
	"Pokémon Moon",
	"Pokémon Ultra Sun",
	"Pokémon Ultra Moon",
	"Pokémon: Let's Go Pikachu!",
	"Pokémon: Let's Go Eevee!",
	"Pokémon Sword",
	"Pokémon Shield",
	"Doom",
	"Doom II",
	"Doom 3: BFG",
	"Quake",
	"Wolfenstein 3D",
	"Wolfenstein: The New Order",
	"Wolfenstein: The Old Blood",
	"Wolfenstein 2: The New Colossus",
	"Puyo Puyo Tetris",
	"Katamari Damacy",
	"Bastion",
	"Transistor",
	"System Shock",
	"BioShock",
	"BioShock Infinite",
	"Deus Ex",
	"Metroid Prime",
	"Metroid Prime 2: Echoes",
	"Metroid Prime 3: Corruption",
	"Super Meat Boy",
	"Super Smash Bros. Brawl",
	"Super Smash Bros. Melee",
	"Super Smash Bros. Ultimate",
	"Thief II",
	"Vampire: The Masquerade – Bloodlines",
	"Myst",
	"Batman: Arkham Asylum",
	"Batman: Arkham City",
	"Dishonored",
	"God of War",
	"LittleBigPlanet",
	"Grim Fandango",
	"Day of the Tentacle",
	"Kingdom Hearts",
	"Kingdom Hearts II",
	"The Last of Us",
	"Perfect Dark",
	"Shadow of the Colossus",
	"Fallout: New Vegas",
	"Fire Emblem Awakening",
	"Banjo-Kazooie: Grunty's Revenge",
	"Banjo-Kazooie: Nuts & Bolts",
	"Firewatch",
	"Mass Effect",
	"Mass Effect 2",
	"Mass Effect 3",
	"Mega Man",
	"Mega Man Battle Network",
	"Bayonetta",
	"Bayonetta 2",
	"The Wonderful 101",
	"Sleeping Dogs",
	"Battletoads",
	"Brütal Legend",
	"Halo Wars",
	"Clannad",
	"Highway Blossoms",
	"Infamous",
	"Crash Bandicoot",
	"Just Cause",
	"Just Cause 2",
	"Devil May Cry",
	"Life is Strange",
	"WarioWare: Smooth Moves",
	"WarioWare: Touched!",
	"Dwarf Fortress",
	"Dungeon Keeper",
	"Minecraft",
	"Fortnite",
	"PlayerUnknown's Battlegrounds",
	"Kirby's Adventure",
	"Kirby's Dream Land",
	"Kirby's Epic Yarn",
	"Yoshi's Wooly World",
	"L.A. Noire",
	"Kid Icarus",
	"Ice Climbers",
	"Crysis",
	"Crysis 2",
	"Hitman: Absolution",
	"Metal Gear Solid",
	"F-Zero",
	"F.E.A.R.",
	"Max Payne",
	"Max Payne 2: The Fall of Max Payne",
	"Max Payne 3",
	"Far Cry",
	"Castle Crashers",
	"Sonic Unleashed",
	"Mirror's Edge",
	"The Witcher",
	"The Witcher 2: Assassins of Kings",
	"The Witcher 3: Wild Hunt",
	"Monster Hunter: World",
	"Call of Duty",
	"Call of Duty 4: Modern Warfare",
	"Call of Duty: Modern Warfare Remastered",
	"Call of Duty: Modern Warfare 2",
	"Call of Duty: Modern Warfare 3",
	"Call of Duty: World at War",
	"Call of Duty: Black Ops",
	"Call of Duty: Black Ops II",
	"Call of Duty: Black Ops III",
	"Call of Duty: Black Ops 4",
	"Shadow Tactics: Blade of the Shogun",
	"Commandos: Behind Enemy Lines",
	"Commandos: Beyond the Call of Duty",
	"Commandos 2: Men of Courage",
	"Commandos 3: Destination Berlin",
	"Commandos: Strike Force",
	"Desperados: Wanted Dead or Alive",
	"Desperados 2: Cooper's Revenge",
	"Helldorado",
	"Desperados III",
	"Diablo",
	"Diablo II",
	"Diablo III",
	"Guitar Hero",
	"Command & Conquer: Red Alert",
	"Command & Conquer: Red Alert 2",
	"Command & Conquer: Red Alert 3",
	"Command & Conquer: Red Alert 2 - Yuris Revenge",
	"Command & Conquer: Generals",
	"Command & Conquer: Generals - Zero Hour",
	"Command & Conquer: Tiberian Sun",
	"Command & Conquer 3: Tiberium Wars",
	"Command & Conquer 4: Tiberian Twilight",
	"Command & Conquer: Rivals",
	"Command & Conquer: Tiberium Alliances",
];
static TITLES_LEN: usize = TITLES.len();

static GENRES: [&'static str; 29] = [
	"First-person shooter",
	"Puzzle",
	"Platformer",
	"Action",
	"Adventure",
	"Real-time strategy",
	"Action role-playing",
	"Dungeon crawl",
	"Roguelike",
	"Tactical role-playing",
	"Sports",
	"Simulation",
	"Fighting",
	"Massively multiplayer online",
	"Stealth",
	"Survival",
	"Rhythm",
	"Survival horror",
	"Text adventure",
	"Visual novel",
	"Real-time strategy",
	"Multiplayer online battle arena",
	"Tower defense",
	"Trivia",
	"Real-time tactics",
	"Hack and slash",
	"Battle royale",
	"Third-person shooter",
	"Music",
];
static GENRES_LEN: usize = GENRES.len();

static PLATFORMS: [&'static str; 31] = [
	"Xbox",
	"Xbox 360",
	"Xbox One",
	"PlayStation",
	"PlayStation 2",
	"PlayStation 3",
	"PlayStation 4",
	"PlayStation Portable",
	"PlayStation Vita",
	"Game Boy",
	"Game Boy Color",
	"Game Boy Advance",
	"Nintendo DS",
	"Nintendo DSi",
	"Nintendo 3DS",
	"Nintendo Entertainment System",
	"Super Nintendo Entertainment System",
	"Nintendo 64",
	"GameCube",
	"Wii",
	"Wii U",
	"Nintendo Switch",
	"Windows",
	"macOS",
	"Linux",
	"Sega Genesis",
	"Sega Master System",
	"Game Gear",
	"Sega Saturn",
	"Sega Dreamcast",
	"Virtual Boy",
];
static PLATFORMS_LEN: usize = PLATFORMS.len();
