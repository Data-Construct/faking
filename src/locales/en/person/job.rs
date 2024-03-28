use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn field() -> String {
	FIELDS[seeder::gen_range(0..FIELDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn seniority() -> String {
	SENIORITY[seeder::gen_range(0..SENIORITY_LEN)].to_string()
}

#[wasm_bindgen]
pub fn position() -> String {
	POSITIONS[seeder::gen_range(0..POSITIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn key_skill() -> String {
	KEY_SKILLS[seeder::gen_range(0..KEY_SKILLS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn employment_type() -> String {
	EMPLOYMENT_TYPES[seeder::gen_range(0..EMPLOYMENT_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn education_level() -> String {
	EDUCATION_LEVELS[seeder::gen_range(0..EDUCATION_LEVELS_LEN)].to_string()
}

static FIELDS: [&'static str; 23] = [
	"Marketing",
	"IT",
	"Accounting",
	"Administration",
	"Advertising",
	"Banking",
	"Community-Services",
	"Construction",
	"Consulting",
	"Design",
	"Education",
	"Farming",
	"Government",
	"Healthcare",
	"Hospitality",
	"Legal",
	"Manufacturing",
	"Marketing",
	"Mining",
	"Real-Estate",
	"Retail",
	"Sales",
	"Technology",
];
static FIELDS_LEN: usize = FIELDS.len();

static SENIORITY: [&'static str; 21] = [
	"Lead",
	"Senior",
	"Product",
	"National",
	"Regional",
	"District",
	"Central",
	"Global",
	"Customer",
	"Investor",
	"Dynamic",
	"International",
	"Legacy",
	"Forward",
	"Internal",
	"Chief",
	"Direct",
	"Corporate",
	"Future",
	"Human",
	"Principal",
];
static SENIORITY_LEN: usize = SENIORITY.len();

static POSITIONS: [&'static str; 25] = [
	"Supervisor",
	"Associate",
	"Executive",
	"Liaison",
	"Officer",
	"Manager",
	"Engineer",
	"Specialist",
	"Director",
	"Coordinator",
	"Administrator",
	"Architect",
	"Analyst",
	"Designer",
	"Planner",
	"Orchestrator",
	"Technician",
	"Developer",
	"Producer",
	"Consultant",
	"Assistant",
	"Facilitator",
	"Agent",
	"Representative",
	"Strategist",
];
static POSITIONS_LEN: usize = POSITIONS.len();

static KEY_SKILLS: [&'static str; 12] = [
	"Teamwork",
	"Communication",
	"Problem solving",
	"Leadership",
	"Organisation",
	"Work under pressure",
	"Confidence",
	"Self-motivated",
	"Networking skills",
	"Proactive",
	"Fast learner",
	"Technical savvy",
];
static KEY_SKILLS_LEN: usize = KEY_SKILLS.len();

static EMPLOYMENT_TYPES: [&'static str; 6] = [
	"Full-time",
	"Part-time",
	"Temporary",
	"Contract",
	"Internship",
	"Commission",
];
static EMPLOYMENT_TYPES_LEN: usize = EMPLOYMENT_TYPES.len();

static EDUCATION_LEVELS: [&'static str; 4] = [
	"Associates",
	"Bachelor",
	"Master",
	"Doctorate",
];
static EDUCATION_LEVELS_LEN: usize = EDUCATION_LEVELS.len();
