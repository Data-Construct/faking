use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn chess_player() -> String {
	PLAYERS[seeder::gen_range(0..PLAYERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn chess_tournaments() -> String {
	TOURNAMENTS[seeder::gen_range(0..TOURNAMENTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn chess_opening() -> String {
	OPENINGS[seeder::gen_range(0..OPENINGS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn chess_titles() -> String {
	TITLES[seeder::gen_range(0..TITLES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn chess_square_name() -> String {
	SQUARE_NAMES[seeder::gen_range(0..SQUARE_NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn chess_piece_name() -> String {
	PIECE_NAMES[seeder::gen_range(0..PIECE_NAMES_LEN)].to_string()
}

static PLAYERS: [&'static str; 20] = [
	"Alexander Alekhine",
	"Alexei Shirov",
	"Alexis Vargas",
	"Anatoly Karpov",
	"Bobby Fischer",
	"Emanuel Lasker",
	"Fabiano Caruana",
	"Garry Kasparov",
	"Hikaru Nakamura",
	"Jose Raul Capablanca",
	"Levon Aronian",
	"Magnus Carlsen",
	"Mikhail Botvinnik",
	"Radjabov Teimour",
	"Sergey Karjakin",
	"Tigran Petrosian",
	"Viswanathan Anand",
	"Vladimir Kramnik",
	"Wesley So",
	"Paul Morphy",
];
static PLAYERS_LEN: usize = PLAYERS.len();

static TOURNAMENTS: [&'static str; 21] = [
	"Wijk aan Zee",
	"Linares",
	"Astrakhan",
	"Dortmund",
	"Shanghai",
	"Bilbao",
	"Nanjing",
	"Moscow",
	"London",
	"Moscow",
	"Tromsø (Chess World Cup)",
	"Paris (Grand Prix 2012–2013)",
	"Bucharest",
	"Nizhny Novgorod (Russian Championship)",
	"Zurich",
	"Khanty-Mansisyk (Candidates Tournament)",
	"Tbilisi (Grand Prix 2014–2015)",
	"Khanty-Mansisyk (Grand Prix 2014–2015)",
	"Baku (Chess World Cup)",
	"London (Grand Chess Tour)",
	"Gibraltar",
];
static TOURNAMENTS_LEN: usize = TOURNAMENTS.len();

static OPENINGS: [&'static str; 42] = [
	"Alekhines Defense",
	"Benko Gambit",
	"Benoni Defense",
	"Birds Opening",
	"Bogo-Indian Defense",
	"Budapest Gambit",
	"Catalan Opening",
	"Caro-Kann Defense",
	"Colle System",
	"Dutch Defense",
	"Giuoco Piano",
	"English Opening",
	"Evans Gambit",
	"Four Knights Game",
	"French Defense",
	"Grünfeld Defense",
	"Italian Game",
	"Kings Gambit",
	"Kings Indian Attack",
	"Kings Indian Defense",
	"Kings Pawn Game",
	"London System",
	"Modern Defense",
	"Nimzo-Indian Defense",
	"Nimzowitsch Defense",
	"Petrovs Defense",
	"Philidors Defense",
	"Pirc Defense",
	"Queens Pawn Game",
	"Queens Gambit Accepted",
	"Queens Gambit Declined",
	"Queens Indian Defense",
	"Réti Opening",
	"Ruy Lopez",
	"Scandinavian Defense",
	"Scotch Game",
	"Sicilian Defense",
	"Slav Defense",
	"Torre Attack",
	"Two Knights Defense",
	"Vienna Game",
	"Wade Defense",
];
static OPENINGS_LEN: usize = OPENINGS.len();

static TITLES: [&'static str; 12] = [
	"GM", "IM", "FM", "CM", "WGM", "WIM", "WFM", "WCM", "AGM", "AIM", "AFM", "ACM",
];
static TITLES_LEN: usize = TITLES.len();

static SQUARE_NAMES: [&'static str; 64] = [
	"A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8",
	"C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8",
	"E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8",
	"G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "H1", "H2", "H3", "H4", "H5", "H6", "H7", "H8",
];
static SQUARE_NAMES_LEN: usize = SQUARE_NAMES.len();

static PIECE_NAMES: [&'static str; 6] = [
	"Pawn",
	"Bishop",
	"Knight",
	"Rook",
	"Queen",
	"King",
];
static PIECE_NAMES_LEN: usize = PIECE_NAMES.len();
