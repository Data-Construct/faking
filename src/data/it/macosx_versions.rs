use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn maxosx_version() -> String {
	MACOSX_VERSIONS[seeder::gen_range(0..MACOSX_VERSIONS_LEN)].to_string()
}

static MACOSX_VERSIONS: [&'static str; 22] = [
	"OS X 10 beta: Kodiak",
	"OS X 10.0: Cheetah",
	"OS X 10.1: Puma",
	"OS X 10.2: Jaguar",
	"OS X 10.3 Panther (Pinot)",
	"OS X 10.4 Tiger (Merlot)",
	"OS X 10.4.4 Tiger (Chardonnay)",
	"OS X 10.5 Leopard (Chablis)",
	"OS X 10.6 Snow Leopard",
	"OS X 10.7 Lion (Barolo)",
	"OS X 10.8 Mountain Lion (Zinfandel)",
	"OS X 10.9 Mavericks (Cabernet)",
	"OS X 10.10: Yosemite (Syrah)",
	"OS X 10.11: El Capitan (Gala)",
	"macOS 10.12: Sierra (Fuji)",
	"macOS 10.13: High Sierra (Lobo)",
	"macOS 10.14: Mojave (Liberty)",
	"macOS 10.15: Catalina (Jazz)",
	"macOS 11: Big Sur (GoldenGate)",
	"macOS 12: Monterey (Star)",
	"macOS 13: Ventura (Rome)",
	"macOS 14: Sonoma (Sunburst)",
];
static MACOSX_VERSIONS_LEN: usize = MACOSX_VERSIONS.len();
