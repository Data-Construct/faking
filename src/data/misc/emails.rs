use crate::misc::usernames;
use rand::Rng;
use wasm_bindgen::prelude::*;

/* standard_generic_email - basically anything username + combo @ domain + ending

standard public email

standard business email

standard gov email

standard account email - takes in first name last name or username generates email



enum - different catgories */

#[wasm_bindgen]
pub fn standard_public_email() -> String {
	let mut rng = rand::thread_rng();
	let username = usernames::random_username();
	let domain = PUBLIC_EMAIL_DOMAINS[rng.gen_range(0..PUBLIC_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_business_email() -> String {
	let mut rng = rand::thread_rng();
	let username = usernames::random_username();
	let domain = BUSINESS_EMAIL_DOMAINS[rng.gen_range(0..BUSINESS_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".com")
}

#[wasm_bindgen]
pub fn standard_government_email() -> String {
	let mut rng = rand::thread_rng();
	let username = usernames::random_username();
	let domain =
		GOVERNMENT_EMAIL_DOMAINS[rng.gen_range(0..GOVERNMENT_EMAIL_DOMAINS_LEN)].to_string();
	format!("{}{}{}{}", username, "@", domain, ".gov")
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
