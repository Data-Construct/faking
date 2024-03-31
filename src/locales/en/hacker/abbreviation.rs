use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = hacker_abbreviation)]
pub fn abbreviation() -> String {
	EN_ABBREVIATION[seeder::gen_range(0..EN_ABBREVIATION_LEN)].to_string()
}

static EN_ABBREVIATION: [&'static str; 42] = [
    "ADP",
    "AGP",
    "AI",
    "API",
    "ASCII",
    "CLI",
    "COM",
    "CSS",
    "DNS",
    "DRAM",
    "EXE",
    "FTP",
    "GB",
    "HDD",
    "HEX",
    "HTTP",
    "IB",
    "IP",
    "JBOD",
    "JSON",
    "OCR",
    "PCI",
    "PNG",
    "RAM",
    "RSS",
    "SAS",
    "SCSI",
    "SDD",
    "SMS",
    "SMTP",
    "SQL",
    "SSD",
    "SSL",
    "TCP",
    "THX",
    "TLS",
    "UDP",
    "USB",
    "UTF8",
    "VGA",
    "XML",
    "XSS",
];
static EN_ABBREVIATION_LEN: usize = EN_ABBREVIATION.len();