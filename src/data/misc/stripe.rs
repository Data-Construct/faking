use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CardData {
	pub vendor: String,
	pub number: String,
}

#[derive(Debug)]
pub struct TokenData {
	pub vendor: String,
	pub name: String,
}

#[derive(Debug)]
pub struct InvalidCardData {
	pub error: String,
	pub number: String,
}

lazy_static! {
	static ref VALID_CARDS_HASHMAP: HashMap<&'static str, String> = {
		let mut card_map = HashMap::new();
		card_map.insert("visa", "4242424242424242".to_owned());
		card_map.insert("mc", "5555555555554444".to_owned());
		card_map.insert("mc_2_series", "2223003122003222".to_owned());
		card_map.insert("mc_debit", "5200828282828210".to_owned());
		card_map.insert("mc_prepaid", "5105105105105100".to_owned());
		card_map.insert("amex", "378282246310005".to_owned());
		card_map.insert("amex_2", "371449635398431".to_owned());
		card_map.insert("discover", "6011111111111117".to_owned());
		card_map.insert("discover_2", "6011000990139424".to_owned());
		card_map.insert("diners_club", "3056930009020004".to_owned());
		card_map.insert("diners_club_2", "36227206271667".to_owned());
		card_map.insert("jcb", "3566002020360505".to_owned());
		card_map
	};
	static ref VALID_TOKENS_HASHMAP: HashMap<&'static str, String> = {
		let mut token_map = HashMap::new();
		token_map.insert("visa", "tok_visa".to_owned());
		token_map.insert("visa_debit", "tok_visa_debit".to_owned());
		token_map.insert("mc", "tok_mastercard".to_owned());
		token_map.insert("mc_debit", "tok_mastercard_debit".to_owned());
		token_map.insert("mc_prepaid", "tok_mastercard_prepaid".to_owned());
		token_map.insert("amex", "tok_amex".to_owned());
		token_map.insert("discover", "tok_discover".to_owned());
		token_map.insert("diners_club", "tok_diners".to_owned());
		token_map.insert("jcb", "tok_jcb".to_owned());
		token_map
	};
	static ref INVALID_CARDS_HASHMAP: HashMap<&'static str, String> = {
		let mut invalid_cards_map = HashMap::new();
		invalid_cards_map.insert("addressZipFail", "4000000000000010".to_owned());
		invalid_cards_map.insert("addressFail", "4000000000000028".to_owned());
		invalid_cards_map.insert("zipFail", "4000000000000036".to_owned());
		invalid_cards_map.insert("addressZipUnavailable", "4000000000000044".to_owned());
		invalid_cards_map.insert("cvcFail", "4000000000000101".to_owned());
		invalid_cards_map.insert("customerChargeFail", "4000000000000341".to_owned());
		invalid_cards_map.insert("successWithReview", "4000000000009235".to_owned());
		invalid_cards_map.insert("declineCard", "4000000000000002".to_owned());
		invalid_cards_map.insert("declineFraudulentCard", "4100000000000019".to_owned());
		invalid_cards_map.insert("declineIncorrectCvc", "4000000000000127".to_owned());
		invalid_cards_map.insert("declineExpired", "4000000000000069".to_owned());
		invalid_cards_map.insert("declineProcessingError", "4000000000000119".to_owned());
		invalid_cards_map.insert("declineIncorrectNumber", "4242424242424241".to_owned());
		invalid_cards_map
	};
}
static VALID_CARD_TYPES: [&'static str; 12] = [
	"visa",
	"mc",
	"mc_2_series",
	"mc_debit",
	"mc_prepaid",
	"amex",
	"amex_2",
	"discover",
	"discover_2",
	"diners_club",
	"diners_club_2",
	"jcb",
];
static VALID_CARD_TYPES_LEN: usize = VALID_CARD_TYPES.len();

static VALID_TOKEN_TYPES: [&'static str; 9] = [
	"visa",
	"visa_debit",
	"mc",
	"mc_debit",
	"mc_prepaid",
	"amex",
	"discover",
	"diners_club",
	"jcb",
];
static VALID_TOKEN_TYPES_LEN: usize = VALID_TOKEN_TYPES.len();

static INVALID_CARD_TYPES: [&'static str; 13] = [
	"addressZipFail",
	"addressFail",
	"zipFail",
	"addressZipUnavailable",
	"cvcFail",
	"customerChargeFail",
	"successWithReview",
	"declineCard",
	"declineFraudulentCard",
	"declineIncorrectCvc",
	"declineExpired",
	"declineProcessingError",
	"declineIncorrectNumber",
];
static INVALID_CARD_TYPES_LEN: usize = INVALID_CARD_TYPES.len();

pub fn valid_card() -> CardData {
	let mut rng = rand::thread_rng();
	let card_type = VALID_CARD_TYPES[rng.gen_range(0..VALID_CARD_TYPES_LEN)].to_string();
	let card = VALID_CARDS_HASHMAP.get(card_type.as_str()).unwrap();

	CardData {
		vendor: card_type,
		number: card.to_owned(),
	}
}

#[wasm_bindgen]
pub fn valid_card_vendor() -> String {
	valid_card().vendor
}

#[wasm_bindgen]
pub fn valid_card_number() -> String {
	valid_card().number
}

pub fn valid_token() -> TokenData {
	let mut rng = rand::thread_rng();
	let token_type = VALID_TOKEN_TYPES[rng.gen_range(0..VALID_TOKEN_TYPES_LEN)].to_string();
	let token = VALID_TOKENS_HASHMAP.get(token_type.as_str()).unwrap();

	TokenData {
		vendor: token_type,
		name: token.to_owned(),
	}
}

#[wasm_bindgen]
pub fn valid_token_vendor() -> String {
	valid_token().vendor
}

#[wasm_bindgen]
pub fn valid_token_name() -> String {
	valid_token().name
}

pub fn invalid_token() -> InvalidCardData {
	let mut rng = rand::thread_rng();
	let invalid_card_type =
		INVALID_CARD_TYPES[rng.gen_range(0..INVALID_CARD_TYPES_LEN)].to_string();
	let number = INVALID_CARDS_HASHMAP
		.get(invalid_card_type.as_str())
		.unwrap();

	InvalidCardData {
		error: invalid_card_type,
		number: number.to_owned(),
	}
}

#[wasm_bindgen]
pub fn invalid_token_error() -> String {
	invalid_token().error
}

pub fn invalid_token_number() -> String {
	invalid_token().number
}
