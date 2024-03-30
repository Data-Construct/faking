use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = vehicle_type)]
pub fn vehicle_type() -> String {
	EN_VEHICLE_TYPE[seeder::gen_range(0..EN_VEHICLE_TYPE_LEN)].to_string()
}

static EN_VEHICLE_TYPE: [&'static str; 11] = [
    "Cargo Van",
    "Convertible",
    "Coupe",
    "Crew Cab Pickup",
    "Extended Cab Pickup",
    "Hatchback",
    "Minivan",
    "Passenger Van",
    "SUV",
    "Sedan",
    "Wagon",
];
static EN_VEHICLE_TYPE_LEN: usize = EN_VEHICLE_TYPE.len();