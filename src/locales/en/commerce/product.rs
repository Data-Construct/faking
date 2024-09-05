use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn product_name() -> String {
	adjective() + " " + &material() + " " + &product()
}

#[wasm_bindgen]
pub fn adjective() -> String {
	EN_ADJECTIVE[seeder::gen_range(0..EN_ADJECTIVE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn material() -> String {
	EN_MATERIAL[seeder::gen_range(0..EN_MATERIAL_LEN)].to_string()
}

#[wasm_bindgen]
pub fn product() -> String {
	EN_PRODUCT[seeder::gen_range(0..EN_PRODUCT_LEN)].to_string()
}

static EN_MATERIAL: [&'static str; 12] = [
    "Steel",
    "Bronze",
    "Wooden",
    "Concrete",
    "Plastic",
    "Cotton",
    "Granite",
    "Rubber",
    "Metal",
    "Soft",
    "Fresh",
    "Frozen",
];
static EN_MATERIAL_LEN: usize = EN_MATERIAL.len();

static EN_ADJECTIVE: [&'static str; 24] = [
    "Small",
    "Ergonomic",
    "Electronic",
    "Rustic",
    "Intelligent",
    "Gorgeous",
    "Incredible",
    "Elegant",
    "Fantastic",
    "Practical",
    "Modern",
    "Recycled",
    "Sleek",
    "Bespoke",
    "Awesome",
    "Generic",
    "Handcrafted",
    "Handmade",
    "Oriental",
    "Licensed",
    "Luxurious",
    "Refined",
    "Unbranded",
    "Tasty",
];
static EN_ADJECTIVE_LEN: usize = EN_ADJECTIVE.len();

static EN_PRODUCT: [&'static str; 24] = [
    "Chair",
    "Car",
    "Computer",
    "Keyboard",
    "Mouse",
    "Bike",
    "Ball",
    "Gloves",
    "Pants",
    "Shirt",
    "Table",
    "Shoes",
    "Hat",
    "Towels",
    "Soap",
    "Tuna",
    "Chicken",
    "Fish",
    "Cheese",
    "Bacon",
    "Pizza",
    "Salad",
    "Sausages",
    "Chips",
];
static EN_PRODUCT_LEN: usize = EN_PRODUCT.len();