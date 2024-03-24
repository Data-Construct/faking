use crate::defaults::usernames;
use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn standard_generic_email() -> String {
	let format = seeder::gen_range(0..4);

	match format {
		0 => standard_public_email_alias(),
		1 => standard_public_email(),
		2 => standard_business_email(),
		3 => standard_government_email(),
		_ => "".to_string(),
	}
}

#[wasm_bindgen]
pub fn standard_public_email() -> String {
	let username = usernames::random_username();
	let domain = PUBLIC_EMAIL_DOMAINS[seeder::gen_range(0..PUBLIC_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_public_email_alias() -> String {
	let format = seeder::gen_range(0..2);
	let username = match format {
		0 => format!(
			"{}{}{}",
			usernames::random_username(),
			"+",
			seeder::gen_range(0..100).to_string()
		),
		1 => format!(
			"{}{}{}",
			usernames::random_username(),
			"+",
			BUSINESS_EMAIL_DOMAINS[seeder::gen_range(0..BUSINESS_EMAIL_DOMAINS_LEN)].to_string()
		),
		_ => "".to_string(),
	};
	let domain = PUBLIC_EMAIL_DOMAINS[seeder::gen_range(0..PUBLIC_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_business_email() -> String {
	let username = usernames::random_username();
	let domain = BUSINESS_EMAIL_DOMAINS[seeder::gen_range(0..BUSINESS_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_government_email() -> String {
	let username = usernames::random_username();
	let domain =
		GOVERNMENT_EMAIL_DOMAINS[seeder::gen_range(0..GOVERNMENT_EMAIL_DOMAINS_LEN)].to_string();
	//Some government emails end with .gov domain
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_account_email(first_name: &str, last_name: &str) -> String {
	let username = usernames::corporate_username_from_input(first_name, last_name);
	let domain = PUBLIC_EMAIL_DOMAINS[seeder::gen_range(0..PUBLIC_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

static PUBLIC_EMAIL_DOMAINS: [&'static str; 6] = [
	"outlook",
	"hotmail",
	"gmail",
	"yahoo",
	"protonmail",
	"zoho",
];
static PUBLIC_EMAIL_DOMAINS_LEN: usize = PUBLIC_EMAIL_DOMAINS.len();

static BUSINESS_EMAIL_DOMAINS: [&'static str; 20] = [
	"google",
	"microsoft",
	"apple",
	"nvidia",
	"meta",
	"facebook",
	"netflix",
	"amazon",
	"slack",
	"amd",
	"intel",
	"hp",
	"ibm",
	"wellsfargo",
	"goldmansachs",
	"janestreet",
	"akumacaptial",
	"sequioacapital",
	"walmart",
	"costco",
];
static BUSINESS_EMAIL_DOMAINS_LEN: usize = BUSINESS_EMAIL_DOMAINS.len();

static GOVERNMENT_EMAIL_DOMAINS: [&'static str; 4] = [
	"fbi", "cia", "nsa", "gov",
];
static GOVERNMENT_EMAIL_DOMAINS_LEN: usize = GOVERNMENT_EMAIL_DOMAINS.len();
