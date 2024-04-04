use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::adjective::adjective;
use super::ingverb::ingverb;
use super::noun::noun;
use super::verb::verb;
use crate::data::base::hacker::abbreviation::abbreviation;

#[wasm_bindgen(js_name = hacker_phrase)]
pub fn phrase() -> String {
    let format = seeder::gen_range(0..8);

	match format {
		0 => format!("If we {} the {}, we can get to the {} {} through the {} {} {}!", verb(), noun(), abbreviation(), noun(), adjective(), abbreviation(), noun()),
        1 => format!("We need to {} the {} {} {}!", verb(), adjective(), abbreviation(), noun()),
        2 => format!("Try to {} the {} {}, maybe it will {} the {} {}!", verb(), abbreviation(), noun(), verb(), adjective(), noun()),
        3 => format!("You can not {} the {} without {} the {} {} {}!", verb(), noun(), ingverb(), adjective(), abbreviation(), noun()),
        4 => format!("Use the {} {} {}, then you can {} the {} {}!", adjective(), abbreviation(), noun(), verb(), adjective(), noun()),
        5 => format!("The {} {} is down, {} the {} {} so we can {} the {} {}!", abbreviation(), noun(), verb(), adjective(), noun(), verb(), abbreviation(), noun()),
        6 => format!("{} the {} wont do anything, we need to {} the {} {} {}!", ingverb(), noun(), verb(), adjective(), abbreviation(), noun()),
        7 => format!("I will {} the {} {} {}, that should {} the {} {}!", verb(), adjective(), abbreviation(), noun(), verb(), abbreviation(), noun()),
        _ => "".to_string(),
	}
}
