use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = app_name)]
pub fn name() -> String {
	EN_NAME[seeder::gen_range(0..EN_NAME_LEN)].to_string()
}

static EN_NAME: [&'static str; 62] = [
    "Redhold",
    "Treeflex",
    "Trippledex",
    "Kanlam",
    "Bigtax",
    "Daltfresh",
    "Toughjoyfax",
    "Mat Lam Tam",
    "Otcom",
    "Tres-Zap",
    "Y-Solowarm",
    "Tresom",
    "Voltsillam",
    "Biodex",
    "Greenlam",
    "Viva",
    "Matsoft",
    "Temp",
    "Zoolab",
    "Subin",
    "Rank",
    "Job",
    "Stringtough",
    "Tin",
    "It",
    "Home Ing",
    "Zamit",
    "Sonsing",
    "Konklab",
    "Alpha",
    "Latlux",
    "Voyatouch",
    "Alphazap",
    "Holdlamis",
    "Zaam-Dox",
    "Sub-Ex",
    "Quo Lux",
    "Bamity",
    "Ventosanzap",
    "Lotstring",
    "Hatity",
    "Tempsoft",
    "Overhold",
    "Fixflex",
    "Konklux",
    "Zontrax",
    "Tampflex",
    "Span",
    "Namfix",
    "Transcof",
    "Stim",
    "Fix San",
    "Sonair",
    "Stronghold",
    "Fintone",
    "Y-find",
    "Opela",
    "Lotlux",
    "Ronstring",
    "Zathin",
    "Duobam",
    "Keylex",
];
static EN_NAME_LEN: usize = EN_NAME.len();