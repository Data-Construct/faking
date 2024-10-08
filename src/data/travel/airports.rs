use crate::utils::seeder;
use wasm_bindgen::prelude::*;

// TODO need help with what to do with flag bytes array
// Maybe some categories? asia, north america.. etc
// Could add more languages

#[wasm_bindgen]
pub fn us_airport() -> String {
	US_AIRPORT_NAMES[seeder::gen_range(0..US_AIRPORT_NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn us_airport_iata() -> String {
	US_AIRPORT_CODES[seeder::gen_range(0..US_AIRPORT_CODES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn eu_airport() -> String {
	EU_AIRPORT_NAMES[seeder::gen_range(0..EU_AIRPORT_NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn eu_airport_iata() -> String {
	EU_AIRPORT_CODES[seeder::gen_range(0..EU_AIRPORT_CODES_LEN)].to_string()
}

static US_AIRPORT_NAMES: [&'static str; 108] = [
	"Hartsfield Jackson Atlanta International Airport",
	"Baltimore/Washington International Airport",
	"Logan International Airport",
	"Charlotte Douglas International Airport",
	"Midway International Airport",
	"O'Hare International Airport",
	"Dallas/Fort Worth International Airport",
	"Denver International Airport",
	"Detroit Metropolitan Airport",
	"Fort Lauderdale Hollywood International Airport",
	"Daniel K. Inouye International Airport",
	"George Bush Intercontinental Airport",
	"Harry Reid International Airport",
	"Los Angeles International Airport",
	"Miami International Airport",
	"Minneapolis Saint Paul International Airport",
	"John F. Kennedy International Airport",
	"LaGuardia Airport",
	"Newark Liberty International Airport",
	"Orlando International Airport",
	"Philadelphia International Airport",
	"Phoenix Sky Harbor International Airport",
	"Portland International Airport",
	"Salt Lake City International Airport",
	"San Diego International Airport",
	"San Francisco International Airport",
	"Seattle Tacoma International Airport",
	"Tampa International Airport",
	"Ronald Reagan Washington National Airport**",
	"Dulles International Airport",
	"Albuquerque International Sunport",
	"Ted Stevens Anchorage International Airport",
	"Austin Bergstrom International Airport",
	"Boise Airport",
	"Buffalo Niagara International Airport",
	"Charleston International Airport",
	"Cincinnati/Northern Kentucky International Airport",
	"Cleveland Hopkins International Airport",
	"John Glenn Columbus International Airport",
	"El Paso International Airport",
	"Southwest Florida International Airport",
	"Gerald R. Ford International Airport",
	"Bradley International Airport",
	"William P. Hobby Airport",
	"Indianapolis International Airport",
	"Jacksonville International Airport",
	"Kansas City International Airport",
	"Louisville Muhammad Ali International Airport",
	"Memphis International Airport",
	"General Mitchell International Airport",
	"Nashville International Airport",
	"Louis Armstrong New Orleans International Airport",
	"Norfolk International Airport",
	"Oakland International Airport",
	"Will Rogers World Airport",
	"Eppley Airfield",
	"Ontario International Airport",
	"John Wayne Airport",
	"Northwest Florida Beaches International Airport",
	"Pittsburgh International Airport",
	"Rhode Island T. F. Green International Airport",
	"Raleigh Durham International Airport",
	"Reno Tahoe International Airport",
	"Richmond International Airport",
	"Sacramento International Airport",
	"San Antonio International Airport",
	"San Jose International Airport",
	"Spokane International Airport",
	"St. Louis Lambert International Airport",
	"Palm Beach International Airport",
	"Albany International Airport",
	"Appleton International Airport",
	"Atlantic City International Airport",
	"Bellingham International Airport",
	"Birmingham Shuttlesworth International Airport",
	"Dayton International Airport",
	"Des Moines International Airport",
	"Fairbanks International Airport",
	"Fresno Yosemite International Airport",
	"Green Bay Austin Straubel International Airport",
	"Piedmont Triad International Airport",
	"Harrisburg International Airport",
	"Hilo International Airport",
	"Huntsville International Airport",
	"Jackson Medgar Wiley Evers International Airport",
	"Key West International Airport",
	"Kona International Airport",
	"McGhee Tyson Airport",
	"Clinton National Airport",
	"Midland International Air and Space Port",
	"Myrtle Beach International Airport",
	"Stewart International Airport",
	"Orlando Sanford International Airport",
	"Palm Springs International Airport",
	"Pensacola International Airport",
	"Phoenix Mesa Gateway Airport",
	"Portland International Jetport",
	"Batten International Airport",
	"Greater Rochester International Airport",
	"Sarasota Bradenton International Airport",
	"Savannah/Hilton Head International Airport",
	"St. Pete Clearwater International Airport",
	"Syracuse Hancock International Airport",
	"Tallahassee International Airport",
	"Tucson International Airport",
	"Tulsa International Airport",
	"Wilkes-Barre/Scranton International Airport",
	"Wilmington International Airport",
];
static US_AIRPORT_NAMES_LEN: usize = US_AIRPORT_NAMES.len();


static US_AIRPORT_CODES: [&'static str; 108] = [
	"ATL",
	"BWI",
	"BOS",
	"CLT",
	"MDW",
	"ORD",
	"DFW",
	"DEN",
	"DTW",
	"FLL",
	"HNL",
	"IAH",
	"LAS",
	"LAX",
	"MIA",
	"MSP",
	"JFK",
	"LGA",
	"EWR",
	"MCO",
	"PHL",
	"PHX",
	"PDX",
	"SLC",
	"SAN",
	"SFO",
	"SEA",
	"TPA",
	"DCA",
	"IAD",
	"ABQ",
	"ANC",
	"AUS",
	"BOI",
	"BUF",
	"CHS",
	"CVG",
	"CLE",
	"CMH",
	"ELP",
	"RSW",
	"GRR",
	"BDL",
	"HOU",
	"IND",
	"JAX",
	"MCI",
	"SDF",
	"MEM",
	"MKE",
	"BNA",
	"MSY",
	"ORF",
	"OAK",
	"OKC",
	"OMA",
	"ONT",
	"SNA",
	"ECP",
	"PIT",
	"PVD",
	"RDU",
	"RNO",
	"RIC",
	"SMF",
	"SAT",
	"SJC",
	"GEG",
	"STL",
	"PBI",
	"ALB",
	"ATW",
	"ACY",
	"BLI",
	"BHM",
	"DAY",
	"DSM",
	"FAI",
	"FAT",
	"GRB",
	"GSO",
	"MDT",
	"ITO",
	"HSV",
	"JAN",
	"EYW",
	"KOA",
	"TYS",
	"LIT",
	"MAF",
	"MYR",
	"SWF",
	"SFB",
	"PSP",
	"PNS",
	"AZA",
	"PWM",
	"RAC",
	"ROC",
	"SRQ",
	"SAV",
	"PIE",
	"SYR",
	"TLH",
	"TUS",
	"TUL",
	"AVP",
	"ILM",
];
static US_AIRPORT_CODES_LEN: usize = US_AIRPORT_CODES.len();


static EU_AIRPORT_NAMES: [&'static str; 74] = [
	"Gatwick Airport",
	"Heathrow Airport",
	"Luton Airport",
	"London Southend Airport",
	"London Stansted Airport",
	"Manchester Airport",
	"Charles de Gaulle Airport",
	"Orly Airport",
	"Amsterdam Airport Schiphol",
	"Karlsruhe/Baden-Baden Airport",
	"Berlin Brandenburg Airport",
	"Düsseldorf Airport",
	"Frankfurt Airport",
	"Munich Airport",
	"Josep Tarradellas Barcelona El Prat Airport",
	"Madrid-Barajas Airport",
	"Palma de Mallorca Airport",
	"Milan Malpensa Airport",
	"Leonardo da Vinci Fiumicino Airport",
	"Athens International Airport",
	"Oslo Airport, Gardermoen",
	"Stockholm Arlanda Airport",
	"Birmingham Airport",
	"Bristol Airport",
	"Liverpool John Lennon Airport",
	"London City Airport",
	"Luton Airport",
	"Newcastle Airport",
	"East Midlands Airport",
	"Aberdeen Airport",
	"Edinburgh Airport",
	"Glasgow International Airport",
	"Belfast International Airport",
	"Beauvais Tillé Airport",
	"Bordeaux Mérignac Airport",
	"Lyon Saint Exupéry Airport",
	"Marseille Provence Airport",
	"EuroAirport Basel Mulhouse Freiburg",
	"Nantes Atlantique Airport",
	"Nice Côte d'Azur Airport",
	"Toulouse Blagnac Airport",
	"Vienna International Airport",
	"Findel Airport",
	"Dublin Airport",
	"Václav Havel Airport Prague",
	"Karlsruhe/Baden-Baden Airport",
	"Cologne/Bonn Airport",
	"EuroAirport Basel Mulhouse Freiburg",
	"Hamburg Airport",
	"Hannover Airport",
	"Stuttgart Airport",
	"Budapest Ferenc Liszt International Airport",
	"Bratislava Airport",
	"Zurich Airport",
	"Kraków John Paul II International Airport",
	"Alicante Elche Miguel Hernández Airport",
	"Bilbao Airport",
	"Fuerteventura Airport",
	"Gran Canaria Airport",
	"Ibiza Airport",
	"Lanzarote Airport",
	"Málaga Airport",
	"Seville Airport",
	"Tenerife North Ciudad de La Laguna Airport",
	"Tenerife South Airport",
	"Valencia Airport",
	"Lisbon Airport",
	"Palese Airport",
	"Orio al Serio Airport",
	"Bologna Airport",
	"Cagliari Airport",
	"Fontanarossa Airport",
	"Linate Airport",
	"Naples International Airport",
];
static EU_AIRPORT_NAMES_LEN: usize = EU_AIRPORT_NAMES.len();

static EU_AIRPORT_CODES: [&'static str; 74] = [
	"LGW",
	"LHR",
	"LTN",
	"SEN",
	"STN",
	"MAN",
	"CDG",
	"ORY",
	"AMS",
	"FKB",
	"BER",
	"DUS",
	"FRA",
	"MUC",
	"BCN",
	"MAD",
	"PMI",
	"MXP",
	"FCO",
	"ATH",
	"OSL",
	"ARN",
	"BHX",
	"BRS",
	"LPL",
	"LCY",
	"LTN",
	"NCL",
	"EMA",
	"ABZ",
	"EDI",
	"GLA",
	"BFS",
	"BVA",
	"BOD",
	"LYS",
	"MRS",
	"BSL",
	"NTE",
	"NCE",
	"TLS",
	"VIE",
	"LUX",
	"DUB",
	"PRG",
	"FKB",
	"CGN",
	"MLH/BSL/EAP",
	"HAM",
	"HAJ",
	"STR",
	"BUD",
	"BTS",
	"ZRH",
	"KRK",
	"ALC",
	"BIO",
	"FUE",
	"LPA",
	"IBZ",
	"ACE",
	"AGP",
	"SVQ",
	"TFN",
	"TFS",
	"VLC",
	"LIS",
	"BRI",
	"BGY",
	"BLQ",
	"CAG",
	"CTA",
	"LIN",
	"NAP",
];
static EU_AIRPORT_CODES_LEN: usize = EU_AIRPORT_CODES.len();
