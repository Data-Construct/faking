use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn industry() -> String {
	let mut rng = rand::thread_rng();
	INDUSTRY[rng.gen_range(0..INDUSTRY_LEN)].to_string()
}

#[wasm_bindgen]
pub fn super_sector() -> String {
	let mut rng = rand::thread_rng();
	SUPER_SECTOR[rng.gen_range(0..SUPER_SECTOR_LEN)].to_string()
}

#[wasm_bindgen]
pub fn sector() -> String {
	let mut rng = rand::thread_rng();
	SECTOR[rng.gen_range(0..SECTOR_LEN)].to_string()
}

#[wasm_bindgen]
pub fn sub_sector() -> String {
	let mut rng = rand::thread_rng();
	SUB_SECTOR[rng.gen_range(0..SUB_SECTOR_LEN)].to_string()
}

static INDUSTRY: [&'static str; 10] = [
  "Oil & Gas",
  "Basic Materials",
  "Industrials",
  "Consumer Goods",
  "Health Care",
  "Consumer Services",
  "Telecommunications",
  "Utilities",
  "Financials",
  "Technology"
];
static INDUSTRY_LEN: usize = INDUSTRY.len();

static SUPER_SECTOR: [&'static str; 19] = [
  "Oil & Gas",
  "Chemicals",
  "Basic Resources",
  "Construction & Materials",
  "Industrial Goods & Services",
  "Automobiles & Parts",
  "Food & Beverage",
  "Personal & Household Goods",
  "Health Care",
  "Retail",
  "Media",
  "Travel & Leisure",
  "Telecommunications",
  "Utilities",
  "Banks",
  "Insurance",
  "Real Estate",
  "Financial Services",
  "Technology"
];
static SUPER_SECTOR_LEN: usize = SUPER_SECTOR.len();

static SECTOR: [&'static str; 41] = [
  "Oil & Gas Producers",
  "Oil Equipment, Services & Distribution",
  "Alternative Energy",
  "Chemicals",
  "Forestry & Paper",
  "Industrial Metals & Mining",
  "Mining",
  "Construction & Materials",
  "Aerospace & Defense",
  "General Industrials",
  "Electronic & Electrical Equipment",
  "Industrial Engineering",
  "Industrial Transportation",
  "Support Services",
  "Automobiles & Parts",
  "Beverages",
  "Food Producers",
  "Household Goods & Home Construction",
  "Leisure Goods",
  "Personal Goods",
  "Tobacco",
  "Health Care Equipment & Services",
  "Pharmaceuticals & Biotechnology",
  "Food & Drug Retailers",
  "General Retailers",
  "Media",
  "Travel & Leisure",
  "Fixed Line Telecommunications",
  "Mobile Telecommunications",
  "Electricity",
  "Gas, Water & Multiutilities",
  "Banks",
  "Nonlife Insurance",
  "Life Insurance",
  "Real Estate Investment & Services",
  "Real Estate Investment Trusts",
  "Financial Services",
  "Equity Investment Instruments",
  "Nonequity Investment Instruments",
  "Software & Computer Services",
  "Technology Hardware & Equipment"
];
static SECTOR_LEN: usize = SECTOR.len();

static SUB_SECTOR: [&'static str; 114] = [
  "Exploration & Production",
  "Integrated Oil & Gas",
  "Oil Equipment & Services",
  "Pipelines",
  "Renewable Energy Equipment",
  "Alternative Fuels",
  "Commodity Chemicals",
  "Specialty Chemicals",
  "Forestry",
  "ÊPaper",
  "Aluminum",
  "Nonferrous Metals",
  "Iron & Steel",
  "Coal",
  "Diamonds & Gemstones",
  "General Mining",
  "Gold Mining",
  "Platinum & Precious Metals",
  "Building Materials & Fixtures",
  "Heavy Construction",
  "Aerospace",
  "Defense",
  "Containers & Packaging",
  "Diversified Industrials",
  "Electrical Components & Equipment",
  "Electronic Equipment",
  "Commercial Vehicles & Trucks",
  "Industrial Machinery",
  "Delivery Services",
  "Marine Transportation",
  "Railroads",
  "Transportation Services",
  "Trucking",
  "Business Support Services",
  "Business Training & Employment Agencies",
  "Financial Administration",
  "Industrial Suppliers",
  "Waste & Disposal Services",
  "Automobiles",
  "Auto Parts",
  "Tires",
  "Brewers",
  "Distillers & Vintners",
  "Soft Drinks",
  "Farming & Fishing",
  "Food Products",
  "Durable Household Products",
  "Nondurable Household Products",
  "Furnishings",
  "Home Construction",
  "Consumer Electronics",
  "Recreational Products",
  "Toys",
  "Clothing & Accessories",
  "Footwear",
  "Personal Products",
  "Tobacco",
  "Health Care Providers",
  "Medical Equipment",
  "Medical Supplies",
  "Biotechnology",
  "Pharmaceuticals",
  "Drug Retailers",
  "Food Retailers & Wholesalers",
  "Apparel Retailers",
  "Broadline Retailers",
  "Home Improvement Retailers",
  "Specialized Consumer Services",
  "Specialty Retailers",
  "Broadcasting & Entertainment",
  "Media Agencies",
  "Publishing",
  "Airlines",
  "Gambling",
  "Hotels",
  "Recreational Services",
  "Restaurants & Bars",
  "Travel & Tourism",
  "Fixed Line Telecommunications",
  "Mobile Telecommunications",
  "Conventional Electricity",
  "Alternative Electricity",
  "Gas Distribution",
  "Multiutilities",
  "Water",
  "Banks",
  "Full Line Insurance",
  "Insurance Brokers",
  "Property & Casualty Insurance",
  "Reinsurance",
  "Life Insurance",
  "Real Estate Holding & Development",
  "Real Estate Services",
  "Industrial & Office REITs",
  "Retail REITs",
  "Residential REITs",
  "Diversified REITs",
  "Specialty REITs",
  "Mortgage REITs",
  "Hotel & Lodging REITs",
  "Asset Managers",
  "Consumer Finance",
  "Specialty Finance",
  "Investment Services",
  "Mortgage Finance",
  "Equity Investment Instruments",
  "Nonequity Investment Instruments",
  "Computer Services",
  "Internet",
  "Software",
  "Computer Hardware",
  "Electronic Office Equipment",
  "Semiconductors",
  "Telecommunications Equipment"
];
static SUB_SECTOR_LEN: usize = SUB_SECTOR.len();