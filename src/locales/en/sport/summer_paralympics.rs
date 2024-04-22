use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sport_summer_paralympics)]
pub fn summer_paralympics_sport() -> String {
	EN_SUMMER_PAEALYMPICS[seeder::gen_range(0..EN_SUMMER_PAEALYMPICS_LEN)].to_string()
}

static EN_SUMMER_PAEALYMPICS: [&'static str; 22] = [
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
static EN_SUMMER_PAEALYMPICS_LEN: usize = EN_SUMMER_PAEALYMPICS.len();