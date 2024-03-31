use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = vehicle_manufacturer)]
pub fn manufacturer() -> String {
	EN_MANUFACTURER[seeder::gen_range(0..EN_MANUFACTURER_LEN)].to_string()
}

static EN_MANUFACTURER: [&'static str; 32] = [
    "Aston Martin",
    "Audi",
    "Bentley",
    "BMW",
    "Bugatti",
    "Cadillac",
    "Chevrolet",
    "Chrysler",
    "Dodge",
    "Ferrari",
    "Fiat",
    "Ford",
    "Honda",
    "Hyundai",
    "Jaguar",
    "Jeep",
    "Kia",
    "Lamborghini",
    "Land Rover",
    "Maserati",
    "Mazda",
    "Mercedes Benz",
    "Mini",
    "Nissan",
    "Polestar",
    "Porsche",
    "Rolls Royce",
    "Smart",
    "Tesla",
    "Toyota",
    "Volkswagen",
    "Volvo",
];
static EN_MANUFACTURER_LEN: usize = EN_MANUFACTURER.len();