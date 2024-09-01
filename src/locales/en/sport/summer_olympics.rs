use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_summer_olympics)]
pub fn summer_olympics_sport() -> String {
	EN_SUMMER_OLYMPICS[seeder::gen_range(0..EN_SUMMER_OLYMPICS_LEN)].to_string()
}

static EN_SUMMER_OLYMPICS: [&'static str; 47] = [
    "3x3 basketball",
    "Archery",
    "Artistic gymnastics",
    "Artistic swimming",
    "Athletics",
    "Badminton",
    "Baseball # Technically part of \"Baseball Softball\" according to IOC website",
    "Basketball",
    "Beach volleyball",
    "BMX freestyle # Officially \"Cycling BMX Freestyle\"",
    "BMX racing # Officially \"Cycling BMX Racing\"",
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
    "Mountain bike # Officially \"Cycling Mountain Bike\"",
    "Rhythmic gymnastics",
    "Road cycling # Officially \"Cycling Road\"",
    "Rowing",
    "Rugby # Officially \"Rugby Sevens\"",
    "Sailing",
    "Shooting",
    "Skateboarding",
    "Softball # Technically part of \"Baseball Softball\" according to IOC website",
    "Sport climbing",
    "Surfing",
    "Swimming",
    "Table tennis",
    "Taekwondo",
    "Tennis",
    "Track cycling # Officially \"Cycling Track\"",
    "Trampoline",
    "Triathlon",
    "Volleyball",
    "Water polo",
    "Weight lifting",
    "Wrestling",
];
static EN_SUMMER_OLYMPICS_LEN: usize = EN_SUMMER_OLYMPICS.len();