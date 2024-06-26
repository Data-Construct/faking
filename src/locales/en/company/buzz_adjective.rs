use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = company_buzz_adjective)]
pub fn buzz_adjective() -> String {
	EN_BUZZ_ADJECTIVE[seeder::gen_range(0..EN_BUZZ_ADJECTIVE_LEN)].to_string()
}

static EN_BUZZ_ADJECTIVE: [&'static str; 65] = [
    "clicks-and-mortar",
    "value-added",
    "vertical",
    "proactive",
    "robust",
    "revolutionary",
    "scalable",
    "leading-edge",
    "innovative",
    "intuitive",
    "strategic",
    "e-business",
    "mission-critical",
    "sticky",
    "one-to-one",
    "24/7",
    "end-to-end",
    "global",
    "B2B",
    "B2C",
    "granular",
    "frictionless",
    "virtual",
    "viral",
    "dynamic",
    "24/365",
    "best-of-breed",
    "killer",
    "magnetic",
    "bleeding-edge",
    "web-enabled",
    "interactive",
    "dot-com",
    "sexy",
    "back-end",
    "real-time",
    "efficient",
    "front-end",
    "distributed",
    "seamless",
    "extensible",
    "turn-key",
    "world-class",
    "open-source",
    "cross-platform",
    "cross-media",
    "synergistic",
    "bricks-and-clicks",
    "out-of-the-box",
    "enterprise",
    "integrated",
    "impactful",
    "wireless",
    "transparent",
    "next-generation",
    "cutting-edge",
    "user-centric",
    "visionary",
    "customized",
    "ubiquitous",
    "plug-and-play",
    "collaborative",
    "compelling",
    "holistic",
    "rich",
];
static EN_BUZZ_ADJECTIVE_LEN: usize = EN_BUZZ_ADJECTIVE.len();