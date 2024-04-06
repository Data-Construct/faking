use wasm_bindgen::prelude::*;
use crate::utils::seeder;
use rand::Rng;
use std::iter;
use chrono::{Datelike, Utc};

#[wasm_bindgen]
pub fn get_cc_number() -> String {
    let cardnum = get_cc_number_and_issuer();
    let cardnum_split = cardnum.split_once(": ").unwrap();
    cardnum_split.1.to_string()
}

#[wasm_bindgen]
pub fn get_cc_issuer() -> String {
    ISSUERS[seeder::gen_range(0..ISSUERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn get_cc_number_and_issuer() -> String {
    let mut card_number = String::from("");
    let issuer = ISSUERS[seeder::gen_range(0..ISSUERS_LEN)];
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;

    match issuer {
        "Visa" => {
            card_number.push('4');
            let num_of_digits = [12,15][seeder::gen_range(0..2)];
            let digits: String = iter::repeat_with(one_char).take(num_of_digits).collect();
            card_number.push_str(&digits);
        },
        "Mastercard" => {
            card_number.push('5');
            let digits: String = iter::repeat_with(one_char).take(15).collect();
            card_number.push_str(&digits);
        },
        "American Express" => {
            card_number.push('3');
            card_number.push(['4','7'][seeder::gen_range(0..2)]);
            let digits: String = iter::repeat_with(one_char).take(13).collect();
            card_number.push_str(&digits);

        },
        "Discover" => {
            card_number.push('6');
            let digits: String = iter::repeat_with(one_char).take(15).collect();
            card_number.push_str(&digits);
        },
        "Diners Club" => {
            card_number.push('3');
            card_number.push(['0','6','8'][seeder::gen_range(0..3)]);
            let digits: String = iter::repeat_with(one_char).take(12).collect();
            card_number.push_str(&digits);
            // Add either 0, 6, 8
        },
        "Carte Blanche" => {
            card_number.push('3');
            card_number.push(['0','6','8'][seeder::gen_range(0..3)]);
            let digits: String = iter::repeat_with(one_char).take(12).collect();
            card_number.push_str(&digits);
            // Add either 0, 6, 8
        },
        _ => {},
    };

    format!("{}: {}", issuer, card_number)
}

#[wasm_bindgen]
pub fn get_cc_cvv() -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    let digits: String = iter::repeat_with(one_char).take(3).collect();
    digits
}

#[wasm_bindgen]
pub fn get_cc_expiry() -> String {
    let mut rng = rand::thread_rng();
    let month = rng.gen_range(1..13);
    let now = Utc::now();
    let year = (now.year() + rng.gen_range(1..6)).to_string().split_off(2);

    format!("{:02}/{}", month, year)
}

static ISSUERS: [&'static str; 6] = [
    "Visa",
    "Mastercard",
    "American Express",
    "Discover",
    "Diners Club",
    "Carte Blanche",
];
static ISSUERS_LEN: usize = ISSUERS.len();