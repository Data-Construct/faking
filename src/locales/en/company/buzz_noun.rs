use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = company_buzz_noun)]
pub fn buzz_noun() -> String {
	EN_BUZZ_NOUN[seeder::gen_range(0..EN_BUZZ_NOUN_LEN)].to_string()
}

static EN_BUZZ_NOUN: [&'static str; 41] = [
    "synergies",
    "paradigms",
    "markets",
    "partnerships",
    "infrastructures",
    "platforms",
    "initiatives",
    "channels",
    "eyeballs",
    "communities",
    "ROI",
    "solutions",
    "action-items",
    "portals",
    "niches",
    "technologies",
    "content",
    "supply-chains",
    "convergence",
    "relationships",
    "architectures",
    "interfaces",
    "e-markets",
    "e-commerce",
    "systems",
    "bandwidth",
    "models",
    "mindshare",
    "deliverables",
    "users",
    "schemas",
    "networks",
    "applications",
    "metrics",
    "e-business",
    "functionalities",
    "experiences",
    "web services",
    "methodologies",
    "blockchains",
    "lifetime value",
];
static EN_BUZZ_NOUN_LEN: usize = EN_BUZZ_NOUN.len();