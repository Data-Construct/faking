use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn plan() -> String {
	PLANS[seeder::gen_range(0..PLANS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn status() -> String {
	STATUSES[seeder::gen_range(0..STATUSES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn payment_method() -> String {
	PAYMENT_METHOD[seeder::gen_range(0..PAYMENT_METHOD_LEN)].to_string()
}

#[wasm_bindgen]
pub fn subscription_term() -> String {
	SUBSCRIPTION_TERMS[seeder::gen_range(0..SUBSCRIPTION_TERMS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn payment_term() -> String {
	PAYMENT_TERMS[seeder::gen_range(0..PAYMENT_TERMS_LEN)].to_string()
}

static PLANS: [&'static str; 14] = [
	"Free Trial",
	"Basic",
	"Starter",
	"Essential",
	"Student",
	"Bronze",
	"Standard",
	"Silver",
	"Gold",
	"Platinum",
	"Professional",
	"Business",
	"Diamond",
	"Premium",
];
static PLANS_LEN: usize = PLANS.len();

static STATUSES: [&'static str; 4] = [
	"Active",
	"Idle",
	"Blocked",
	"Pending",
];
static STATUSES_LEN: usize = STATUSES.len();

static PAYMENT_METHOD: [&'static str; 12] = [
	"Credit card",
	"Debit card",
	"Paypal",
	"Cash",
	"Money transfer",
	"Bitcoins",
	"Cheque",
	"Apple Pay",
	"Google Pay",
	"WeChat Pay",
	"Alipay",
	"Visa checkout",
];
static PAYMENT_METHOD_LEN: usize = PAYMENT_METHOD.len();

static SUBSCRIPTION_TERMS: [&'static str; 8] = [
	"Daily",
	"Weekly",
	"Monthly",
	"Annual",
	"Biennal",
	"Triennal",
	"Quinquennal",
	"Lifetime",
];
static SUBSCRIPTION_TERMS_LEN: usize = SUBSCRIPTION_TERMS.len();

static PAYMENT_TERMS: [&'static str; 4] = [
	"Payment in advance",
	"Monthly",
	"Annual",
	"Full subscription",
];
static PAYMENT_TERMS_LEN: usize = PAYMENT_TERMS.len();
