use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn summer_olympic() -> String {
	SUMMER_OLYMPICS[seeder::gen_range(0..SUMMER_OLYMPICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn winter_olympic() -> String {
	WINTER_OLYMPICS[seeder::gen_range(0..WINTER_OLYMPICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn summer_paralympic() -> String {
	SUMMER_PARALYMPICS[seeder::gen_range(0..SUMMER_PARALYMPICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn winter_paralympics() -> String {
	WINTER_PARALYMPICS[seeder::gen_range(0..WINTER_PARALYMPICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn ancient_olympic() -> String {
	ANCIENT_OLYMPICS[seeder::gen_range(0..ANCIENT_OLYMPICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn unusual() -> String {
	UNUSUAL[seeder::gen_range(0..UNUSUAL_LEN)].to_string()
}
static SUMMER_OLYMPICS: [&'static str; 47] = [
	"3x3 basketball",
	"Archery",
	"Artistic gymnastics",
	"Artistic swimming",
	"Athletics",
	"Badminton",
	"Baseball",
	"Basketball",
	"Beach volleyball",
	"BMX freestyle",
	"BMX racing",
	"Boxing",
	"Canoe/kayak flatwater",
	"Canoe/kayak slalom",
	"Diving",
	"Equestrian",
	"Fencing",
	"Football",
	"Golf",
	"Handball",
	"Hockey",
	"Judo",
	"Karate",
	"Marathon swimming",
	"Modern pentathlon",
	"Mountain bike",
	"Rhythmic gymnastics",
	"Road cycling",
	"Rowing",
	"Rugby",
	"Sailing",
	"Shooting",
	"Skateboarding",
	"Softball",
	"Sport climbing",
	"Surfing",
	"Swimming",
	"Table tennis",
	"Taekwondo",
	"Tennis",
	"Track cycling",
	"Trampoline",
	"Triathlon",
	"Volleyball",
	"Water polo",
	"Weight lifting",
	"Wrestling",
];
static SUMMER_OLYMPICS_LEN: usize = SUMMER_OLYMPICS.len();

static WINTER_OLYMPICS: [&'static str; 16] = [
	"Alpine skiing",
	"Biathlon",
	"Bobsleigh",
	"Cross-country skiing",
	"Curling",
	"Figure skating",
	"Freestyle skiing",
	"Ice hockey",
	"Luge",
	"Nordic combined",
	"Short track speed skating",
	"Skeleton",
	"Ski jumping",
	"Ski mountaineering",
	"Snowboard",
	"Speed skating",
];
static WINTER_OLYMPICS_LEN: usize = WINTER_OLYMPICS.len();

static SUMMER_PARALYMPICS: [&'static str; 22] = [
	"Archery",
	"Athletics",
	"Badminton",
	"Blind football",
	"Boccia",
	"Canoe",
	"Cycling",
	"Equestrian",
	"Goalball",
	"Judo",
	"Powerlifting",
	"Rowing",
	"Shooting",
	"Sitting volleyball",
	"Swimming",
	"Table tennis",
	"Taekwondo",
	"Triathlon",
	"Wheelchair basketball",
	"Wheelchair fencing",
	"Wheelchair rugby",
	"Wheelchair tennis",
];
static SUMMER_PARALYMPICS_LEN: usize = SUMMER_PARALYMPICS.len();

static WINTER_PARALYMPICS: [&'static str; 6] = [
	"Alpine skiing",
	"Biathlon",
	"Cross-country skiing",
	"Para ice hockey",
	"Snowboard",
	"Wheelchair curling",
];
static WINTER_PARALYMPICS_LEN: usize = WINTER_PARALYMPICS.len();

static ANCIENT_OLYMPICS: [&'static str; 9] = [
	"Boxing",
	"Chariot racing",
	"Discus",
	"Horse racing",
	"Long jump",
	"Pankration",
	"Pentathlon",
	"Running",
	"Wrestling",
];
static ANCIENT_OLYMPICS_LEN: usize = ANCIENT_OLYMPICS.len();

static UNUSUAL: [&'static str; 21] = [
	"Apple Racing",
	"Ban'ei",
	"Bathtubbing",
	"Bed racing",
	"Bossaball",
	"Botaoshi",
	"Beer Can Regatta",
	"Black pudding throwing",
	"Bog snorkelling",
	"Bottle kicking",
	"Camel jumping",
	"Camel wrestling",
	"Chess boxing",
	"Extreme ironing",
	"Flugtag/Birdman",
	"Gurning",
	"Kastenlauf (Beer crate running)",
	"Oil wrestling",
	"Poohsticks",
	"Wife carrying",
	"Zorbing",
];
static UNUSUAL_LEN: usize = UNUSUAL.len();
