use rand::Rng;
use wasm_bindgen::prelude::*;

//TODO kevinly77: Full product object?
#[wasm_bindgen]
pub fn department() -> String {
	let mut rng = rand::thread_rng();
	DEPARTMENTS[rng.gen_range(0..DEPARTMENTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn product_adjective() -> String {
	let mut rng = rand::thread_rng();
	PRODUCT_ADJECTIVES[rng.gen_range(0..PRODUCT_ADJECTIVES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn material() -> String {
	let mut rng = rand::thread_rng();
	MATERIALS[rng.gen_range(0..MATERIALS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn product() -> String {
	let mut rng = rand::thread_rng();
	PRODUCTS[rng.gen_range(0..PRODUCTS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn promotion_code_adjective() -> String {
	let mut rng = rand::thread_rng();
	PROMOTION_CODE_ADJECTIVES[rng.gen_range(0..PROMOTION_CODE_ADJECTIVES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn promotion_code_noun() -> String {
	let mut rng = rand::thread_rng();
	PROMOTION_CODE_NOUNS[rng.gen_range(0..PROMOTION_CODE_NOUNS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn brand_commerce() -> String {
	let mut rng = rand::thread_rng();
	BRANDS[rng.gen_range(0..BRANDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn vendor() -> String {
	let mut rng = rand::thread_rng();
	VENDORS[rng.gen_range(0..VENDORS_LEN)].to_string()
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
	let brand = brand_commerce();
	let vendor = vendor();
	return format!(
		"Department: {}\n Product: {}\n Promotion Code: {}\n Brand: {}\n Vendor: {}",
		department, full_product_name, promotion_code, brand, vendor
	);
}

static DEPARTMENTS: [&'static str; 22] = [
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
static DEPARTMENTS_LEN: usize = DEPARTMENTS.len();

static PRODUCT_ADJECTIVES: [&'static str; 17] = [
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
static PRODUCT_ADJECTIVES_LEN: usize = PRODUCT_ADJECTIVES.len();

static MATERIALS: [&'static str; 17] = [
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
static MATERIALS_LEN: usize = MATERIALS.len();

static PRODUCTS: [&'static str; 20] = [
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
static PRODUCTS_LEN: usize = PRODUCTS.len();

static PROMOTION_CODE_ADJECTIVES: [&'static str; 11] = [
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
static PROMOTION_CODE_ADJECTIVES_LEN: usize = PROMOTION_CODE_ADJECTIVES.len();

static PROMOTION_CODE_NOUNS: [&'static str; 8] = [
	"Code",
	"Deal",
	"Discount",
	"Price",
	"Promo",
	"Promotion",
	"Sale",
	"Savings",
];
static PROMOTION_CODE_NOUNS_LEN: usize = PROMOTION_CODE_NOUNS.len();

static BRANDS: [&'static str; 10] = [
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
static BRANDS_LEN: usize = BRANDS.len();

static VENDORS: [&'static str; 4] = [
	"Amazon",
	"Dollar General",
	"Walmart",
	"Target",
];
static VENDORS_LEN: usize = VENDORS.len();
