use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn model_name() -> String {
	MODEL_NAME[seeder::gen_range(0..MODEL_NAME_LEN)].to_string()
}

#[wasm_bindgen]
pub fn manufacturer() -> String {
	MANUFACTURER[seeder::gen_range(0..MANUFACTURER_LEN)].to_string()
}

#[wasm_bindgen]
pub fn serial() -> String {
	SERIAL[seeder::gen_range(0..SERIAL_LEN)].to_string()
}

static MODEL_NAME: [&'static str; 57] = [
	"Galaxy S1",
	"Galaxy S2",
	"Galaxy S3 Mini",
	"Galaxy S3",
	"Galaxy S4 Mini",
	"Galaxy S4",
	"Galaxy S5",
	"Galaxy S6 Edge",
	"Galaxy S7 Edge",
	"Galaxy S7",
	"Galaxy S8 Plus",
	"Galaxy S8",
	"Galaxy S9",
	"Mate 10 Pro",
	"Mate 20 Lite",
	"Mate 20 Pro",
	"Mate 20",
	"Mate 9",
	"Mi 8 Lite",
	"Mi 8 Pro",
	"Mi 8 SE",
	"Mi 8",
	"OnePlus 2",
	"OnePlus 3",
	"OnePlus 3T",
	"OnePlus 5",
	"OnePlus 5T",
	"OnePlus 6",
	"OnePlus 6T",
	"OnePlus One",
	"P Smart",
	"P10 Plus",
	"P10",
	"P20 Pro",
	"P20",
	"Pixel 2 XL",
	"Pixel 2",
	"Pixel 3 XL",
	"Pixel 3",
	"Pixel XL",
	"Pixel",
	"Pocophone F1",
	"Redmi Note 6 Pro",
	"iPhone 3G",
	"iPhone 3GS",
	"iPhone 4",
	"iPhone 4S",
	"iPhone 5",
	"iPhone 5C",
	"iPhone 5S",
	"iPhone 6 / 6 Plus",
	"iPhone 6S / 6S Plus",
	"iPhone 7 / 7 Plus",
	"iPhone 8 / 8 Plus",
	"iPhone SE",
	"iPhone X",
	"iPhone",
];
static MODEL_NAME_LEN: usize = MODEL_NAME.len();

static MANUFACTURER: [&'static str; 11] = [
	"ASUS",
	"Acer",
	"Apple",
	"Dell",
	"Google",
	"HP",
	"Huawei",
	"Lenovo",
	"OnePlus",
	"Samsung",
	"Xiamomi",
];
static MANUFACTURER_LEN: usize = MANUFACTURER.len();

static SERIAL: [&'static str; 26] = [
	"pEekWH7zGxVITv6NTa5KHjLSwr5Ie4",
	"UVr864F8zUbyYOAUd4cFOW9hpsZuGn",
	"Kl2ZroV9a",
	"m6aHiiHOc",
	"hHhDJaHCO",
	"SJMZOmtU0csrv4R",
	"PTIA6Ff3GBvGh3j",
	"hrR8nflThDDaSXO",
	"OezkV3nTii0sMK0",
	"T6UuMUTani3VGY4vXGia",
	"NjGU0z33pgE4sTEED7VR",
	"05skEogwZlX7j6twhhXX",
	"ToFVWLzGTJhQxAaJlDDn",
	"ejfjnRNInxh0363JC2WM",
	"xC36G3Xy4n2Fu90keaW96c1Hw5QBJX",
	"CdNevWfqDPQw4iJgUhtyCqwCggV12T",
	"9vxM9fCsG9nXg8EjTN5ygV2LvaDZdG",
	"39gPmcOKpwhDezLdiIOZ7SH89Pbjp4",
	"Yr9Vt13BlgvXO9zgJTPuCLv6F82r5S",
	"trDuJXhT8PnD3JEtw4lsluEuYSn1Xh",
	"VMTnd2mMQWvjbtNcZh7UIdULKb1mMo",
	"Pbn8^`LkKPi4QJ79xk3SnW_DbV*CL",
	"Hk4BoF=%ud/PV4K-/NvEq28Q/&HNq7",
	"_usDGH^%4dSYvUtH39qu3*hbi3V+z",
	"&XwUS6yR2N&+Z`$94eK@XzFp/p/ie^",
	"tL&^J@24CVF=zP46Lxixk`_a#=o6c5",
];
static SERIAL_LEN: usize = SERIAL.len();
