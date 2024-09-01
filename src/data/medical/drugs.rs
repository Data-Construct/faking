use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn random_drug() -> String {
	let format = seeder::gen_range(0..4);

	match format {
		0 => random_depressants(),
		1 => random_stimulants(),
		2 => random_psychedelics_dissociatives(),
		3 => random_narcotics(),
		_ => "".to_string(),
	}
}

#[wasm_bindgen]
pub fn random_depressants() -> String {
	DEPRESSANTS[seeder::gen_range(0..DEPRESSANTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_stimulants() -> String {
	STIMULANTS[seeder::gen_range(0..STIMULANTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_psychedelics_dissociatives() -> String {
	PSYCHEDELICS[seeder::gen_range(0..PSYCHEDELICS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_narcotics() -> String {
	NARCOTICS[seeder::gen_range(0..NARCOTICS_LEN)].to_string()
}

static DEPRESSANTS: [&'static str; 11] = [
    "Alcohol",
    "Barbiturates",
    "Benzodiazepines",
    "cannabis",
    "carbamates",
    "Gabapentinoids",
    "Gamma-hydroxybutyric acid",
    "Nonbenzodiazepines",
    "Opioids/opiates",
    "Piperidinediones",
    "Quinazolinone",
];
static DEPRESSANTS_LEN: usize = DEPRESSANTS.len();

static STIMULANTS: [&'static str; 27] = [
    "Caffeine",
    "MDMA",
    "Methamphetamine",
    "Adderall",
    "Ritalin",
    "Dextrostat",
    "Dexedrine",
    "Dexedrine Spansule",
    "Zenzedi",
    "Concerta",
    "ProCentra",
    "Vyvanse",
    "Focalin",
    "Strattera",
    "Desoxyn",
    "Ephedrine",
    "MDPV",
    "Mephedrone",
    "Methylphenidate",
    "Cocaine",
    "Nicotine",
    "Phenylpropanolamine",
    "Lisdexamfetamine",
    "Pseudoephedrine",
    "Khat",
    "Modafinil",
    "Pitolisant",
];
static STIMULANTS_LEN: usize = STIMULANTS.len();

static PSYCHEDELICS: [&'static str; 13] = [
    "2C-B",
    "DMT",
    "LSD",
    "Mescaline",
    "Psilocin",
    "Psilocybin",
    "Ketamine",
    "Methoxetamine",
    "PCP",
    "DXM",
    "Nitrous oxide",
    "Scopolamine",
    "atropine",
];
static PSYCHEDELICS_LEN: usize = PSYCHEDELICS.len();

static NARCOTICS: [&'static str; 14] = [
    "Opium",
    "Morphine",
    "Codeine",
    "Heroin",
    "Dextromethorphan",
    "Dextropropoxyphene",
    "Loperamide",
    "Hydrocodone",
    "Oxycodone",
    "Oxymorphone",
    "Meperidine",
    "Methadone",
    "Fentanyl",
    "Carfentanyl",
];
static NARCOTICS_LEN: usize = NARCOTICS.len();