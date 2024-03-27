use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn army_rank() -> String {
	ARMY_RANKS[seeder::gen_range(0..ARMY_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn marine_rank() -> String {
	MARINE_RANKS[seeder::gen_range(0..MARINE_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn navy_rank() -> String {
	NAVY_RANKS[seeder::gen_range(0..NAVY_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn coast_guard_rank() -> String {
	COAST_GUARD_RANKS[seeder::gen_range(0..COAST_GUARD_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn air_force_rank() -> String {
	AIR_FORCE_RANKS[seeder::gen_range(0..AIR_FORCE_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn space_force_rank() -> String {
	SPACE_FORCE_RANKS[seeder::gen_range(0..SPACE_FORCE_RANKS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn dod_paygrade() -> String {
	DOD_PAYGRADES[seeder::gen_range(0..DOD_PAYGRADES_LEN)].to_string()
}

static ARMY_RANKS: [&'static str; 24] = [
	"Private",
	"Private First Class",
	"Specialist",
	"Corporal",
	"Sergeant",
	"Staff Sergeant",
	"Sergeant First Class",
	"Master Sergeant",
	"First Sergeant",
	"Sergeant Major",
	"Command Sergeant Major",
	"Sergeant Major of the Army",
	"Second Lieutenant",
	"First Lieutenant",
	"Captain",
	"Major",
	"Lieutenant Colonel",
	"Colonel",
	"Brigadier General",
	"Major General",
	"Lieutenant General",
	"General",
	"General of the Army",
	"General of the Armies",
];
static ARMY_RANKS_LEN: usize = ARMY_RANKS.len();

static MARINE_RANKS: [&'static str; 22] = [
	"Private",
	"Private First Class",
	"Lance Corporal",
	"Corporal",
	"Sergeant",
	"Staff Sergeant",
	"Gunnery Sergeant",
	"Master Sergeant",
	"First Sergeant",
	"Master Gunnery Sergeant",
	"Sergeant Major",
	"Sergeant Major of the Marine Corps",
	"Second Lieutenant",
	"First Lieutenant",
	"Captain",
	"Major",
	"Lieutenant Colonel",
	"Colonel",
	"Brigadier General",
	"Major General",
	"Lieutenant General",
	"General",
];
static MARINE_RANKS_LEN: usize = MARINE_RANKS.len();

static NAVY_RANKS: [&'static str; 33] = [
	"Seaman Recruit",
	"Fireman Recruit",
	"Airman Recruit",
	"Constructionman Recruit",
	"Seaman Apprentice",
	"Fireman Apprentice",
	"Airman Apprentice",
	"Constructionman Apprentice",
	"Seaman",
	"Fireman",
	"Airman",
	"Constructionman",
	"Petty Officer Third Class",
	"Petty Officer Second Class",
	"Petty Officer First Class",
	"Chief Petty Officer",
	"Senior Chief Petty Officer",
	"Command Senior Chief Petty Officer",
	"Master Chief Petty Officer",
	"Command Master Chief Petty Officer",
	"Fleet Master Chief Petty Officer",
	"Force Master Chief Petty Officer",
	"Master Chief Petty Officer of the Navy",
	"Ensign",
	"Lieutenant",
	"Lieutenant Commander",
	"Commander",
	"Captain",
	"Rear Admiral",
	"Vice Admiral",
	"Admiral",
	"Fleet Admiral",
	"Admiral of the Navy",
];
static NAVY_RANKS_LEN: usize = NAVY_RANKS.len();

static COAST_GUARD_RANKS: [&'static str; 23] = [
	"Seaman Recruit",
	"Seaman Apprentice",
	"Seaman",
	"Petty Officer Third Class",
	"Petty Officer Second Class",
	"Petty Officer First Class",
	"Chief Petty Officer",
	"Senior Chief Petty Officer",
	"Master Chief Petty Officer",
	"Command Master Chief Petty Officer",
	"Area Command Master Chief Petty Officer",
	"Coast Guard Reserve Force Master Chief Petty Officer",
	"Master Chief Petty Officer of the Coast Guard",
	"Ensign",
	"Lieutenant",
	"Lieutenant Commander",
	"Commander",
	"Captain",
	"Rear Admiral",
	"Vice Admiral",
	"Admiral",
	"Fleet Admiral",
	"Admiral of the Navy",
];
static COAST_GUARD_RANKS_LEN: usize = COAST_GUARD_RANKS.len();

static AIR_FORCE_RANKS: [&'static str; 21] = [
	"Airman Basic",
	"Airman First Class",
	"Senior Airman",
	"Staff Sergeant",
	"Technical Sergeant",
	"Master Sergeant",
	"Senior Master Sergeant",
	"Chief Master Sergeant",
	"Command Chief Master Sergeant",
	"Chief Master Sergeant of the Air Force",
	"Second Lieutenant",
	"First Lieutenant",
	"Captain",
	"Major",
	"Lieutenant Colonel",
	"Colonel",
	"Brigadier General",
	"Major General",
	"Lieutenant General",
	"General",
	"General of the Air Force",
];
static AIR_FORCE_RANKS_LEN: usize = AIR_FORCE_RANKS.len();

static SPACE_FORCE_RANKS: [&'static str; 21] = [
	"Airman Basic",
	"Airman First Class",
	"Senior Airman",
	"Staff Sergeant",
	"Technical Sergeant",
	"Master Sergeant",
	"Senior Master Sergeant",
	"Chief Master Sergeant",
	"Command Chief Master Sergeant",
	"Senior Enlisted Advisor of the Space Force",
	"Senior Enlisted Advisor to the Chairman",
	"Second Lieutenant",
	"First Lieutenant",
	"Captain",
	"Major",
	"Lieutenant Colonel",
	"Colonel",
	"Brigadier General",
	"Major General",
	"Lieutenant General",
	"General",
];
static SPACE_FORCE_RANKS_LEN: usize = SPACE_FORCE_RANKS.len();

static DOD_PAYGRADES: [&'static str; 28] = [
	"E-1",
	"E-2",
	"E-3",
	"E-4",
	"E-5",
	"E-6",
	"E-7",
	"E-8",
	"E-9",
	"O-1",
	"O-1E",
	"O-2",
	"O-2E",
	"O-3",
	"O-3E",
	"O-4",
	"O-5",
	"O-6",
	"O-7",
	"O-8",
	"O-9",
	"O-10",
	"W-1",
	"W-2",
	"W-3",
	"W-4",
	"W-5",
	"Special",
];
static DOD_PAYGRADES_LEN: usize = DOD_PAYGRADES.len();
