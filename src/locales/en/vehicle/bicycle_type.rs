use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = vehicle_bicycle_type)]
pub fn bicycle_type() -> String {
	EN_BICYCLE_TYPE[seeder::gen_range(0..EN_BICYCLE_TYPE_LEN)].to_string()
}

static EN_BICYCLE_TYPE: [&'static str; 18] = [
    "Adventure Road Bicycle",
    "BMX Bicycle",
    "City Bicycle",
    "Cruiser Bicycle",
    "Cyclocross Bicycle",
    "Dual-Sport Bicycle",
    "Fitness Bicycle",
    "Flat-Foot Comfort Bicycle",
    "Folding Bicycle",
    "Hybrid Bicycle",
    "Mountain Bicycle",
    "Recumbent Bicycle",
    "Road Bicycle",
    "Tandem Bicycle",
    "Touring Bicycle",
    "Track/Fixed-Gear Bicycle",
    "Triathlon/Time Trial Bicycle",
    "Tricycle",
];
static EN_BICYCLE_TYPE_LEN: usize = EN_BICYCLE_TYPE.len();