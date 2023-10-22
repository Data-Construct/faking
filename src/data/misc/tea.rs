use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn black_tea() -> String {
	let mut rng = rand::thread_rng();
	BLACK[rng.gen_range(0..BLACK_LEN)].to_string()
}

#[wasm_bindgen]
pub fn oolong_tea() -> String {
	let mut rng = rand::thread_rng();
	OOLONG[rng.gen_range(0..OOLONG_LEN)].to_string()
}

#[wasm_bindgen]
pub fn green_tea() -> String {
	let mut rng = rand::thread_rng();
	GREEN[rng.gen_range(0..GREEN_LEN)].to_string()
}

#[wasm_bindgen]
pub fn white_tea() -> String {
	let mut rng = rand::thread_rng();
	WHITE[rng.gen_range(0..WHITE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn herbal_tea() -> String {
	let mut rng = rand::thread_rng();
	HERBAL[rng.gen_range(0..HERBAL_LEN)].to_string()
}

#[wasm_bindgen]
pub fn tea_type() -> String {
	let mut rng = rand::thread_rng();
	TEA_TYPES[rng.gen_range(0..TEA_TYPES_LEN)].to_string()
}
static BLACK: [&'static str; 24] = [
	"Assam",
	"Ceylon",
	"Congou",
	"Darjeeling",
	"Dianhong",
	"Earl Grey",
	"English Afternoon",
	"English Breakfast",
	"Irish Breakfast",
	"Jaekseol",
	"Jiu Qu Hong Mei",
	"Kangra",
	"Keemun",
	"Lady Grey",
	"Lahijan",
	"Lapsang Souchong",
	"Masala Chai",
	"Munnar",
	"Nepali",
	"Nilgiri",
	"Rize",
	"Scottish Breakfast",
	"Sun Moon Lake",
	"Yingdehong",
];
static BLACK_LEN: usize = BLACK.len();

static OOLONG: [&'static str; 20] = [
	"Alishan",
	"Bai Jiguan",
	"Da Hong Pao",
	"Dancong",
	"Dongding",
	"Dongfang Meiren",
	"Fujian",
	"Gaoshan",
	"Huangjin Gui",
	"Ji Xuan",
	"Lishan",
	"Pouchong",
	"Rougui",
	"Ruan Zhi",
	"Shui Jin Gui",
	"Shui Xian",
	"Tieguanyin",
	"Tieluohan",
	"Tienguanyin",
	"Vietnamese",
];
static OOLONG_LEN: usize = OOLONG.len();

static GREEN: [&'static str; 23] = [
	"Bancha",
	"Biluochun",
	"Chun Mee",
	"Daejak",
	"Garucha",
	"Genmaicha",
	"Gunpowder",
	"Gyokuro",
	"Hojicha",
	"Huangshan Maofeng",
	"Ipcha",
	"Jungjak",
	"Kabusecha",
	"Kukicha",
	"Longjing",
	"Lu'an Melon Seed",
	"Matcha",
	"Sejak",
	"Sencha",
	"Shincha",
	"Taipin Houkui",
	"Ujeon",
	"Xinyang Maojian",
];
static GREEN_LEN: usize = GREEN.len();

static WHITE: [&'static str; 5] = [
	"Bai Mu Dan",
	"Fujian New Craft",
	"Gongmei",
	"Shou Mei",
	"Yi Zhen Bai Hao",
];
static WHITE_LEN: usize = WHITE.len();

static HERBAL: [&'static str; 85] = [
	"Anise",
	"Asiatic Penny-Wort",
	"Bael Fruit",
	"Barley",
	"Bee Balm",
	"Boldo",
	"Burdock",
	"Cacao",
	"Caraway",
	"Cat's Claw",
	"Catnip",
	"Cerasse",
	"Chamomile",
	"Che Dang",
	"Chinese Knot-Weed",
	"Chrysanthemum",
	"Cinnamon",
	"Citrus Peel",
	"Dandelion",
	"Dill",
	"Dried Lime",
	"Echinacea",
	"Elderberry",
	"Essiac",
	"European Mistletoe",
	"Fennel",
	"Gentian",
	"Ginger Root",
	"Ginseng",
	"Goji",
	"Hawthorn",
	"Hibiscus",
	"Honeybush",
	"Horehound",
	"Houttuynia",
	"Jiaogulan",
	"Kapor",
	"Kuzuyu",
	"Labrador",
	"Lemon Balm",
	"Lemon Ginger",
	"Lemon Grass",
	"Licorice Root",
	"Lime Blossom",
	"Luo Han Guo",
	"Mint",
	"Moringa",
	"Mountain Tea",
	"Neem",
	"Nettle",
	"New Jersey Tea",
	"Noni",
	"Oksusu Cha",
	"Olive Leaf",
	"Osmanthus",
	"Pandan",
	"Patchouli",
	"Pine",
	"Qishr",
	"Red Clover",
	"Red Raspberry",
	"Roasted Wheat",
	"Rooibos",
	"Rose Hip",
	"Roselle",
	"Rosemary",
	"Sage",
	"Sagebrush",
	"Serendib",
	"Skurayu",
	"Sobacha",
	"Spearmint",
	"Spicebush",
	"Spruce",
	"St. John's Wort",
	"Thyme",
	"Tulsi",
	"Turmeric",
	"Valerian",
	"Verbena",
	"Vetiver",
	"Wax Gourd",
	"Wong Lo Kat",
	"Woodruff",
	"Yarrow",
];
static HERBAL_LEN: usize = HERBAL.len();

static TEA_TYPES: [&'static str; 5] = [
	"Black",
	"Oolong",
	"Green",
	"White",
	"Herbal",
];
static TEA_TYPES_LEN: usize = TEA_TYPES.len();
