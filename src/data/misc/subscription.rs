use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn plan() -> String {
	let mut rng = rand::thread_rng();
	PLANS[rng.gen_range(0..PLANS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn status() -> String {
	let mut rng = rand::thread_rng();
	STATUSES[rng.gen_range(0..STATUSES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn payment_method() -> String {
	let mut rng = rand::thread_rng();
	PAYMENT_METHOD[rng.gen_range(0..PAYMENT_METHOD_LEN)].to_string()
}

#[wasm_bindgen]
pub fn subscription_term() -> String {
	let mut rng = rand::thread_rng();
	SUBSCRIPTION_TERMS[rng.gen_range(0..SUBSCRIPTION_TERMS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn payment_term() -> String {
	let mut rng = rand::thread_rng();
	PAYMENT_TERMS[rng.gen_range(0..PAYMENT_TERMS_LEN)].to_string()
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
