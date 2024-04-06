use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn random_vehicle_style() -> String {
	let format = FORMATS[seeder::gen_range(0..FORMATS_LEN)].to_string();
    let mut licensePlate = String::from("");

    for i in 0..format.len() {
        let char = format.chars().nth(i).unwrap();
        match char {
            '?' => {
                const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                let mut rng = rand::thread_rng();
                let one_char = CHARSET[rng.gen_range(0..CHARSET.len())] as char;
                licensePlate.push(one_char);
            },
            '#' => {
                let num = seeder::gen_range(0..10).to_string();
                licensePlate.push_str(&num);
            },
            _ => {licensePlate.push(char)}
        }
    }
    licensePlate
}

static FORMATS: [&'static str; 39] = [
    "???-####",
    "#??####",
    "##??###",
    "??? ###",
    "???####",
    "### ???",
    "#### ??",
    "####??",
    "#???###",
    "???-###",
    "??·#####",
    "######",
    "??? ?##",
    "### #??",
    "???####",
    "??? ###",
    "#? #####",
    "#? ?####",
    "#? ??###",
    "#? #?###",
    "#? ####?",
    "#? ###??",
    "? ######",
    "? ######",
    "??# ####",
    "?? #####",
    "###?",
    "###??",
    "###???",
    "####??",
    "#####?",
    "#### ???",
    "??###",
    "####",
    "#-#####",
    "?## #??",
    "??-###",
    "#-#####?",
    "??# ?#?",
];
static FORMATS_LEN: usize = FORMATS.len();