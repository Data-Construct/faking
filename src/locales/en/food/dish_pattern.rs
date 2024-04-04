use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::adjective::adjective;
use super::ethnic_category::ethnic_category;
use super::fruit::fruit;
use super::meat::meat;
use super::vegetable::vegetable;
use super::ingredient::ingredient;
use super::spice::spice;
use crate::locales::en::person::name::first_name;

#[wasm_bindgen(js_name = food_dish_pattern)]
pub fn dish_parttern() -> String {
	let format = seeder::gen_range(0..15);

	match format {
		0 => format!("{} {} stew", adjective(), ethnic_category()).to_string(),
        1 => format!("{} {} with {}", adjective(), meat(), vegetable()).to_string(),
        2 => format!("{} {} soup", ethnic_category(), ingredient()).to_string(),
        3 => format!("{} and {} tart", fruit(), fruit()).to_string(),
        4 => format!("{} pie", fruit()).to_string(),
        5 => format!("{}-glazed {} skewers", fruit(), meat()).to_string(),
        6 => format!("{}-infused {} roast", fruit(), meat()).to_string(),
        7 => format!("{} and {} pie", ingredient(), meat()).to_string(),
        8 => format!("{}-infused {}", ingredient(), meat()).to_string(),
        9 => format!("{} steak", meat()).to_string(),
        10 => format!("{} with {} sauce", meat(), fruit()).to_string(),
        11 => format!("{}-crusted {}", spice(), meat()).to_string(),
        12 => format!("{}-rubbed {} salad", spice(), meat()).to_string(),
        13 => format!("{} salad", vegetable()).to_string(),
        14 => format!("Special {} from {}", ingredient(), first_name()).to_string(),
        _ => "".to_string(),
	}
}
