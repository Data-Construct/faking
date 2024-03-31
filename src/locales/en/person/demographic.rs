use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn race() -> String {
	RACE[seeder::gen_range(0..RACE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn sex() -> String {
	SEX[seeder::gen_range(0..SEX_LEN)].to_string()
}

#[wasm_bindgen]
pub fn demonym() -> String {
	DEMONYM[seeder::gen_range(0..DEMONYM_LEN)].to_string()
}

#[wasm_bindgen]
pub fn educational_attainment() -> String {
	EDUCATIONAL_ATTAINMENT[seeder::gen_range(0..EDUCATIONAL_ATTAINMENT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn martial_status() -> String {
	MARTIAL_STATUS[seeder::gen_range(0..MARTIAL_STATUS_LEN)].to_string()
}

static RACE: [&'static str; 5] = [
	"American Indian or Alaska Native",
	"Asian",
	"Black or African American",
	"Native Hawaiian or Other Pacific Islander",
	"White",
];
static RACE_LEN: usize = RACE.len();

static SEX: [&'static str; 2] = [
	"Male",
	"Female",
];
static SEX_LEN: usize = SEX.len();

static DEMONYM: [&'static str; 188] = [
	"Afghan",
	"Albanian",
	"Algerian",
	"American",
	"Andorran",
	"Angolan",
	"Argentine",
	"Armenian",
	"Aromanian",
	"Aruban",
	"Australian",
	"Austrian",
	"Azerbaijani",
	"Bahamian",
	"Bahraini",
	"Bangladeshi",
	"Barbadian",
	"Basotho",
	"Basque",
	"Belarusian",
	"Belgian",
	"Belizean",
	"Bermudian",
	"Bissau-Guinean",
	"Boer",
	"Bosniak",
	"Brazilian",
	"Breton",
	"Briton",
	"British Virgin Islander",
	"Bruneian",
	"Bulgarian",
	"Burkinabè",
	"Burundian",
	"Cambodian",
	"Cameroonian",
	"Canadian",
	"Catalan",
	"Cape Verdean",
	"Chadian",
	"Chilean",
	"Chinese",
	"Colombian",
	"Comorian",
	"Congolese",
	"Croatian",
	"Cuban",
	"Cypriot",
	"Czech",
	"Dane",
	"Dominican",
	"Dutch",
	"East Timorese",
	"Ecuadorian",
	"Egyptian",
	"Emirati",
	"English",
	"Eritrean",
	"Estonian",
	"Ethiopian",
	"Falkland Islander",
	"Faroese",
	"Finn",
	"Fijian",
	"Filipino",
	"French",
	"Georgian",
	"German",
	"Ghanaian",
	"Gibraltar",
	"Greek",
	"Grenadian",
	"Guatemalan",
	"French Guianan",
	"Guinean",
	"Guyanese",
	"Haitian",
	"Honduran",
	"Hong Konger",
	"Hungarian",
	"Icelander",
	"I-Kiribati",
	"Indian",
	"Indonesian",
	"Iranian",
	"Iraqi",
	"Irish",
	"Israeli",
	"Italian",
	"Ivoirian",
	"Jamaican",
	"Japanese",
	"Jordanian",
	"Kazakh",
	"Kenyan",
	"Korean",
	"Kosovar",
	"Kurd",
	"Kuwaiti",
	"Kyrgyz",
	"Lao",
	"Latvian",
	"Lebanese",
	"Liberian",
	"Libyan",
	"Liechtensteiner",
	"Lithuanian",
	"Luxembourger",
	"Macanese",
	"Macedonian",
	"Malagasy",
	"Malaysian",
	"Malawian",
	"Maldivian",
	"Malian",
	"Maltese",
	"Manx",
	"Mauritian",
	"Mexican",
	"Moldovan",
	"Moroccan",
	"Mongolian",
	"Montenegrin",
	"Namibian",
	"Nepalese",
	"New Zealander",
	"Nicaraguan",
	"Nigerien",
	"Nigerian",
	"Norwegian",
	"Pakistani",
	"Palauan",
	"Palestinian",
	"Panamanian",
	"Papua New Guinean",
	"Paraguayan",
	"Peruvian",
	"Pole",
	"Portuguese",
	"Puerto Rican",
	"Quebecer",
	"Romanian",
	"Russian",
	"Rwandan",
	"Salvadoran",
	"São Toméan",
	"Saudi",
	"Scottish",
	"Senegalese",
	"Serb",
	"Sierra Leonean",
	"Singaporean",
	"Sindhian",
	"Slovak",
	"Slovene",
	"Somali",
	"Somalilander",
	"South African",
	"Spaniard",
	"Sri Lankan",
	"St Lucian",
	"Sudanese",
	"Surinamese",
	"Swede",
	"Swiss",
	"Syriac",
	"Syrian",
	"Tajik",
	"Taiwanese",
	"Tanzanian",
	"Thai",
	"Tibetan",
	"Tobagonian",
	"Trinidadian",
	"Tunisian",
	"Turk",
	"Tuvaluan",
	"Ugandan",
	"Ukrainian",
	"Uruguayan",
	"Uzbek",
	"Vanuatuan",
	"Venezuelan",
	"Vietnamese",
	"Welsh",
	"Yemeni",
	"Zambian",
	"Zimbabwean",
];
static DEMONYM_LEN: usize = DEMONYM.len();

static EDUCATIONAL_ATTAINMENT: [&'static str; 13] = [
	"No schooling completed",
	"Nursery school",
	"Kindergarten",
	"Grade 1 though 11",
	"12th grade - No Diploma",
	"Regular high school diploma",
	"GED or alternative credential",
	"Some college",
	"Associates degree",
	"Bachelors degree",
	"Masters degree",
	"Professional degree",
	"Doctorate degree",
];
static EDUCATIONAL_ATTAINMENT_LEN: usize = EDUCATIONAL_ATTAINMENT.len();

static MARTIAL_STATUS: [&'static str; 5] = [
	"Married",
	"Widowed",
	"Divorced",
	"Separated",
	"Never married",
];
static MARTIAL_STATUS_LEN: usize = MARTIAL_STATUS.len();
