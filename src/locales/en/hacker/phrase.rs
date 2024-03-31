use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::adjective::adjective;
use super::ingverb::ingverb;
use super::noun::noun;
use super::verb::verb;
use super::abbreviation::abbreviation;

fn template(verb: &str, noun: &str, abbreviation: &str, adjective: &str, ingverb: &str) -> Vec<String> {
    vec![
        format!("If we {} the {}, we can get to the {} {} through the {} {} {}!", verb, noun, abbreviation, noun, adjective, abbreviation, noun),
        format!("We need to {} the {} {} {}!", verb, adjective, abbreviation, noun),
        format!("Try to {} the {} {}, maybe it will {} the {} {}!", verb, abbreviation, noun, verb, adjective, noun),
        format!("You can't {} the {} without {} the {} {} {}!", verb, noun, ingverb, adjective, abbreviation, noun),
        format!("Use the {} {} {}, then you can {} the {} {}!", adjective, abbreviation, noun, verb, adjective, noun),
        format!("The {} {} is down, {} the {} {} so we can {} the {} {}!", abbreviation, noun, verb, adjective, noun, verb, abbreviation, noun),
        format!("{} the {} won't do anything, we need to {} the {} {} {}!", ingverb, noun, verb, adjective, abbreviation, noun),
        format!("I'll {} the {} {} {}, that should {} the {} {}!", verb, adjective, abbreviation, noun, verb, abbreviation, noun),
    ]
}

#[wasm_bindgen(js_name = hacker_phrase)]
pub fn phrase() -> String {
    let adjective = adjective();
    let ingverb = ingverb();
    let noun = noun();
    let verb = verb();
    let abbreviation = abbreviation();

    template(&verb, &noun, &abbreviation, &adjective, &ingverb)[seeder::gen_range(0..8)].to_string()
}