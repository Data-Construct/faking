use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::adjective::adjective;
use super::ethnic_category::ethnic_category;
use super::fruit::fruit;
use super::ingredient::ingredient;
use super::meat::meat;
use super::spice::spice;
use super::vegetable::vegetable;

use crate::locales::en::animal::bird::bird;
use crate::locales::en::location::country::country;
use crate::locales::en::color::human::human;


#[wasm_bindgen(js_name = food_description_parttern)]
pub fn description_parttern() -> String {
	let format = seeder::gen_range(0..21);

	match format {
		0 => format!("A classic pie filled with delicious {} and {} {}, baked in a {} pastry crust and topped with a golden-brown lattice.", meat(), adjective(), ingredient(), adjective()).to_string(),
        1 => format!("A delightful tart combining {} {} and sweet {}, set in a buttery pastry shell and finished with a hint of {}.", adjective(), vegetable(), fruit(), spice()).to_string(),
        2 => format!("A heartwarming {} soup, featuring fresh {} and an aromatic blend of traditional spices.", ethnic_category(), ingredient()).to_string(),
        3 => format!("A robust {} stew featuring {} flavors, loaded with {} meat, {} vegetables, and a {}, {} broth.", adjective(), ethnic_category(), adjective(), adjective(), adjective(), adjective()).to_string(),
        4 => format!("A simple {} pie. No fancy stuff. Just pie.", fruit()).to_string(),
        5 => format!("A slow-roasted {} with a {}, {} exterior. Stuffed with {} and covered in {} sauce. Sides with {} puree and wild {}.", bird(), adjective(), adjective(), fruit(), fruit(), vegetable(), vegetable()).to_string(),
        6 => format!("A special {} {} from {}. To support the strong flavor it is sided with a tablespoon of {}.", human(), ingredient(), country(), spice()).to_string(),
        7 => format!("A succulent {} steak, encased in a {} {} crust, served with a side of {} mashed {}.", meat(), adjective(), spice(), spice(), vegetable()).to_string(),
        8 => format!("An exquisite {} roast, infused with the essence of {}, slow-roasted to bring out its natural flavors and served with a side of creamy {}.", meat(), fruit(), vegetable()).to_string(),
        9 => format!("Baked {}-stuffed {}, seasoned with {} and {} herbs, accompanied by roasted {} medley.", ingredient(), meat(), spice(), adjective(), vegetable()).to_string(),
        10 => format!("Crispy fried {} bites, seasoned with {} and served with a tangy {} dipping sauce.", meat(), spice(), fruit()).to_string(),
        11 => format!("Fresh mixed greens tossed with {}-rubbed {}, {}, and a light dressing.", spice(), meat(), vegetable()).to_string(),
        12 => format!("Fresh {} with a pinch of {}, topped by a caramelized {} with whipped cream.", ingredient(), spice(), fruit()).to_string(),
        13 => format!("Grilled {} kebabs, marinated in {} spices and served with a fresh {} and {} salad.", meat(), ethnic_category(), vegetable(), fruit()).to_string(),
        14 => format!("Hearty {} and {} stew, slow-cooked with {} and {} for a comforting, flavorful meal.", ingredient(), meat(), spice(), vegetable()).to_string(),
        15 => format!("Juicy {}, grilled to your liking and drizzled with a bold {} sauce, served alongside roasted {}.", meat(), spice(), vegetable()).to_string(),
        16 => format!("Our {} {}, slow-cooked to perfection, accompanied by steamed {} and a rich, savory gravy.", adjective(), meat(), vegetable()).to_string(),
        17 => format!("Tender {} skewers, glazed with a sweet and tangy {} sauce, served over a bed of fragrant jasmine rice.", meat(), fruit()).to_string(),
        18 => format!("Tenderly braised {} in a rich {} and {} sauce, served with a side of creamy {}.", meat(), spice(), vegetable(), vegetable()).to_string(),
        19 => format!("Three {} with {}, {}, {}, {} and {}. With a side of baked {}, and your choice of {} or {}.", ingredient(), vegetable(), vegetable(), vegetable(), vegetable(), ingredient(), fruit(), ingredient(), ingredient()).to_string(),
        20 => format!("{}-day aged {} steak, with choice of {} sides.", seeder::gen_range(1..100), meat(), seeder::gen_range(2..5)).to_string(),
        _ => "".to_string(),
	}
}
