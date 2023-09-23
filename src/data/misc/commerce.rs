use rand::Rng;
use wasm_bindgen::prelude::*;

//TODO kevinly77: Full product object?
#[wasm_bindgen]
pub fn department() -> String {
	let mut rng = rand::thread_rng();
	DEPARTMENT[rng.gen_range(0..DEPARTMENT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn product_adjective() -> String {
	let mut rng = rand::thread_rng();
	PRODUCT_ADJECTIVE[rng.gen_range(0..PRODUCT_ADJECTIVE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn material() -> String {
	let mut rng = rand::thread_rng();
	MATERIAL[rng.gen_range(0..MATERIAL_LEN)].to_string()
}

#[wasm_bindgen]
pub fn product() -> String {
	let mut rng = rand::thread_rng();
	PRODUCT[rng.gen_range(0..PRODUCT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn promotion_code_adjective() -> String {
	let mut rng = rand::thread_rng();
	PROMOTION_CODE_ADJECTIVE[rng.gen_range(0..PROMOTION_CODE_ADJECTIVE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn promotion_code_noun() -> String {
	let mut rng = rand::thread_rng();
	PROMOTION_CODE_NOUN[rng.gen_range(0..PROMOTION_CODE_NOUN_LEN)].to_string()
}

#[wasm_bindgen]
pub fn brand() -> String {
	let mut rng = rand::thread_rng();
	BRAND[rng.gen_range(0..BRAND_LEN)].to_string()
}

#[wasm_bindgen]
pub fn vendor() -> String {
	let mut rng = rand::thread_rng();
	VENDOR[rng.gen_range(0..VENDOR_LEN)].to_string()
}

#[wasm_bindgen]
pub fn full_product_name() -> String {
	let product_adjective = product_adjective();
	let product_material = material();
	let product_type = product();
	return format!(
		"{} {} {}",
		product_adjective, product_material, product_type
	);
}

#[wasm_bindgen]
pub fn full_promotion_code() -> String {
	let promotion_code_adjective = promotion_code_adjective();
	let promotion_code_noun = promotion_code_noun();
	return format!("{} {}", promotion_code_adjective, promotion_code_noun,);
}

#[wasm_bindgen]
pub fn full_product_info() -> String {
	let department = department();
	let full_product_name = full_product_name();
	let promotion_code = full_promotion_code();
	let brand = brand();
	let vendor = vendor();
	return format!(
		"Department: {}\n Product: {}\n Promotion Code: {}\n Brand: {}\n Vendor: {}",
		department, full_product_name, promotion_code, brand, vendor
	);
}

static DEPARTMENT: [&'static str; 22] = [
	"Books",
	"Movies",
	"Music",
	"Games",
	"Electronics",
	"Computers",
	"Home",
	"Garden",
	"Tools",
	"Grocery",
	"Health",
	"Beauty",
	"Toys",
	"Kids",
	"Baby",
	"Clothing",
	"Shoes",
	"Jewelry",
	"Sports",
	"Outdoors",
	"Automotive",
	"Industrial",
];

static DEPARTMENT_LEN: usize = DEPARTMENT.len();

static PRODUCT_ADJECTIVE: [&'static str; 17] = [
	"Small",
	"Ergonomic",
	"Rustic",
	"Intelligent",
	"Gorgeous",
	"Incredible",
	"Fantastic",
	"Practical",
	"Sleek",
	"Awesome",
	"Enormous",
	"Mediocre",
	"Synergistic",
	"Heavy Duty",
	"Lightweight",
	"Aerodynamic",
	"Durable",
];

static PRODUCT_ADJECTIVE_LEN: usize = PRODUCT_ADJECTIVE.len();

static MATERIAL: [&'static str; 17] = [
	"Steel",
	"Wooden",
	"Concrete",
	"Plastic",
	"Cotton",
	"Granite",
	"Rubber",
	"Leather",
	"Silk",
	"Wool",
	"Linen",
	"Marble",
	"Iron",
	"Bronze",
	"Copper",
	"Aluminum",
	"Paper",
];

static MATERIAL_LEN: usize = MATERIAL.len();

static PRODUCT: [&'static str; 20] = [
	"Chair",
	"Car",
	"Computer",
	"Gloves",
	"Pants",
	"Shirt",
	"Table",
	"Shoes",
	"Hat",
	"Plate",
	"Knife",
	"Bottle",
	"Coat",
	"Lamp",
	"Keyboard",
	"Bag",
	"Bench",
	"Clock",
	"Watch",
	"Wallet",
];

static PRODUCT_LEN: usize = PRODUCT.len();

static PROMOTION_CODE_ADJECTIVE: [&'static str; 11] = [
	"Amazing",
	"Awesome",
	"Cool",
	"Good",
	"Great",
	"Incredible",
	"Killer",
	"Premium",
	"Special",
	"Stellar",
	"Sweet",
];

static PROMOTION_CODE_ADJECTIVE_LEN: usize = PROMOTION_CODE_ADJECTIVE.len();

static PROMOTION_CODE_NOUN: [&'static str; 8] = [
	"Code",
	"Deal",
	"Discount",
	"Price",
	"Promo",
	"Promotion",
	"Sale",
	"Savings",
];

static PROMOTION_CODE_NOUN_LEN: usize = PROMOTION_CODE_NOUN.len();

static BRAND: [&'static str; 10] = [
	"Samsung",
	"Dell",
	"Nike",
	"Apple",
	"LG",
	"Adidas",
	"Nikon",
	"Sony",
	"Beats",
	"GoPro",
];

static BRAND_LEN: usize = BRAND.len();

static VENDOR: [&'static str; 4] = [
	"Amazon",
	"Dollar General",
	"Walmart",
	"Target",
];

static VENDOR_LEN: usize = VENDOR.len();
