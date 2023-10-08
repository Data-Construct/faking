use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn brand() -> String {
	let mut rng = rand::thread_rng();
	BRANDS[rng.gen_range(0..BRANDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn equipment() -> String {
	let mut rng = rand::thread_rng();
	EQUIPMENT[rng.gen_range(0..EQUIPMENT_LEN)].to_string()
}

static BRANDS: [&'static str; 14] = [
	"Admiral",
	"Amana",
	"Blue Star",
	"Bosch",
	"Electrolux",
	"Fagor",
	"Franke",
	"IKEA",
	"KitchenAid",
	"LG",
	"Samsung",
	"Sharp",
	"Siemens",
	"Whirlpool",
];
static BRANDS_LEN: usize = BRANDS.len();

static EQUIPMENT: [&'static str; 65] = [
	"Air ioniser",
	"Air purifier",
	"Appliance plug",
	"Aroma lamp",
	"Attic fan",
	"Bachelor griller",
	"Back boiler",
	"Beverage opener",
	"Blender",
	"Box mangle",
	"Can opener",
	"Ceiling fan",
	"Central vacuum cleaner",
	"Clothes dryer",
	"Clothes iron",
	"Cold-pressed juicer",
	"Combo washer dryer",
	"Dish draining closet",
	"Dishwasher",
	"Domestic robot",
	"Drawer dishwasher",
	"Electric water boiler",
	"Evaporative cooler",
	"Exhaust hood",
	"Fan heater",
	"Flame supervision device",
	"Forced-air",
	"Futon dryer",
	"Garbage disposal unit",
	"Gas appliance",
	"Go-to-bed matchbox",
	"HVAC",
	"Hair dryer",
	"Hair iron",
	"Hob (hearth)",
	"Home server",
	"Humidifier",
	"Icebox",
	"Kimchi refrigerator",
	"Mangle (machine)",
	"Micathermic heater",
	"Microwave oven",
	"Mousetrap",
	"Oil heater",
	"Oven",
	"Paper shredder",
	"Patio heater",
	"Radiator (heating)",
	"Refrigerator",
	"Sewing machine",
	"Solar water heater",
	"Space heater",
	"Steam mop",
	"Stove",
	"Sump pump",
	"Television",
	"Tie press",
	"Toaster and toaster ovens",
	"Trouser press",
	"Vacuum cleaner",
	"Washing machine",
	"Water cooker",
	"Water heater",
	"Water purifier",
	"Window fan",
];
static EQUIPMENT_LEN: usize = EQUIPMENT.len();
