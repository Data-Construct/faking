use crate::utils::seeder;
use wasm_bindgen::prelude::*;

use crate::locales::en::person::name::{last_name, neutral_first_name};

#[wasm_bindgen(js_name = usa_city_prefix)]
pub fn city_prefix() -> String {
    CITYPREFIXES[seeder::gen_range(0..CITYPREFIXES_LEN)].to_string()
}

#[wasm_bindgen(js_name = usa_city_suffix)]
pub fn city_suffix() -> String {
    CITYSUFFIXES[seeder::gen_range(0..CITYSUFFIXES_LEN)].to_string()
}

#[wasm_bindgen(js_name = usa_street_suffix)]
pub fn street_suffix() -> String {
    STREETSUFFIXES[seeder::gen_range(0..STREETSUFFIXES_LEN)].to_string()
}

#[wasm_bindgen(js_name = usa_street_name)]
pub fn street_name() -> String {
    let format = seeder::gen_range(0..2);

    match format{
        0=>concat_string!(neutral_first_name(), " ", street_suffix()),
        1=>concat_string!(last_name(), " ", street_suffix()),
        _=>"".to_string()
    }
}

#[wasm_bindgen(js_name = usa_state)]
pub fn state() -> String {
    STATES[seeder::gen_range(0..STATES_LEN)].to_string()
}

#[wasm_bindgen(js_name = usa_city)]
pub fn city() -> String {
    let format = seeder::gen_range(0..4);

    match format{
        0=>concat_string!(city_prefix(), " ", neutral_first_name(), city_suffix()),
        1=>concat_string!(city_prefix(), " ", neutral_first_name()),
        2=>concat_string!(neutral_first_name(), city_suffix()),
        3=>concat_string!(last_name(), city_suffix()),
        _=>"".to_string()
    }
}

#[wasm_bindgen(js_name = usa_zip_code)]
pub fn zip_code(state: &str) -> String {
    let zip_format = match &state as &str{
        "AL"=>"350",
        "AK"=>"995",
        "AS"=>"967",
        "AZ"=>"850",
        "AR"=>"717",
        "CA"=>"900",
        "CO"=>"800",
        "CT"=>"061",
        "DC"=>"204",
        "DE"=>"198",
        "FL"=>"322",
        "GA"=>"301",
        "HI"=>"967",
        "ID"=>"832",
        "IL"=>"600",
        "IN"=>"463",
        "IA"=>"510",
        "KS"=>"666",
        "KY"=>"404",
        "LA"=>"701",
        "ME"=>"042",
        "MD"=>"210",
        "MA"=>"026",
        "MI"=>"480",
        "MN"=>"555",
        "MS"=>"387",
        "MO"=>"650",
        "MT"=>"590",
        "NE"=>"688",
        "NV"=>"898",
        "NH"=>"036",
        "NJ"=>"076",
        "NM"=>"880",
        "NY"=>"122",
        "NC"=>"288",
        "ND"=>"586",
        "OH"=>"444",
        "OK"=>"730",
        "OR"=>"979",
        "PA"=>"186",
        "RI"=>"029",
        "SC"=>"299",
        "SD"=>"577",
        "TN"=>"383",
        "TX"=>"798",
        "UT"=>"847",
        "VT"=>"050",
        "VA"=>"222",
        "WA"=>"990",
        "WV"=>"247",
        "WI"=>"549",
        "WY"=>"831",
        _=>"",
    };
    concat_string!(zip_format, seeder::gen_range(10..99).to_string())
}

#[wasm_bindgen(js_name = usa_street_address)]
pub fn street_address() -> String {
    let format = seeder::gen_range(3..6);

    match format{
        5=>concat_string!(seeder::gen_range(10000..99999).to_string(), " ", street_name()),
        4=>concat_string!(seeder::gen_range(1000..9999).to_string(), " ", street_name()),
        3=>concat_string!(seeder::gen_range(100..999).to_string(), " ", street_name()),
        _=>"".to_string()
    }
}

#[wasm_bindgen(js_name = usa_full_address)]
pub fn full_address() -> String {
    let format = seeder::gen_range(0..2);
    let state = state();
    let zipcode = zip_code(&state);

    match format{
      0=>format!("{}, {}, {} {}", street_address(), city(), state, zipcode),
      1=>format!("{} {}, {}, {} {}", street_address(), secondary_address(), city(), state, zipcode),
        _=>"".to_string()
    }
}

#[wasm_bindgen(js_name = usa_secondary_address)]
pub fn secondary_address() -> String {
  concat_string!(SECONDARY_FORMATS[seeder::gen_range(0..SECONDARY_FORMATS_LEN)].to_string(), " ", seeder::gen_range(100..999).to_string())
}

static STREETSUFFIXES: [&'static str; 225] = [
    "Alley",
    "Avenue",
    "Branch",
    "Bridge",
    "Brook",
    "Brooks",
    "Burg",
    "Burgs",
    "Bypass",
    "Camp",
    "Canyon",
    "Cape",
    "Causeway",
    "Center",
    "Centers",
    "Circle",
    "Circles",
    "Cliff",
    "Cliffs",
    "Club",
    "Common",
    "Corner",
    "Corners",
    "Course",
    "Court",
    "Courts",
    "Cove",
    "Coves",
    "Creek",
    "Crescent",
    "Crest",
    "Crossing",
    "Crossroad",
    "Curve",
    "Dale",
    "Dam",
    "Divide",
    "Drive",
    "Drive",
    "Drives",
    "Estate",
    "Estates",
    "Expressway",
    "Extension",
    "Extensions",
    "Fall",
    "Falls",
    "Ferry",
    "Field",
    "Fields",
    "Flat",
    "Flats",
    "Ford",
    "Fords",
    "Forest",
    "Forge",
    "Forges",
    "Fork",
    "Forks",
    "Fort",
    "Freeway",
    "Garden",
    "Gardens",
    "Gateway",
    "Glen",
    "Glens",
    "Green",
    "Greens",
    "Grove",
    "Groves",
    "Harbor",
    "Harbors",
    "Haven",
    "Heights",
    "Highway",
    "Hill",
    "Hills",
    "Hollow",
    "Inlet",
    "Inlet",
    "Island",
    "Island",
    "Islands",
    "Islands",
    "Isle",
    "Isle",
    "Junction",
    "Junctions",
    "Key",
    "Keys",
    "Knoll",
    "Knolls",
    "Lake",
    "Lakes",
    "Land",
    "Landing",
    "Lane",
    "Light",
    "Lights",
    "Loaf",
    "Lock",
    "Locks",
    "Locks",
    "Lodge",
    "Lodge",
    "Loop",
    "Mall",
    "Manor",
    "Manors",
    "Meadow",
    "Meadows",
    "Mews",
    "Mill",
    "Mills",
    "Mission",
    "Mission",
    "Motorway",
    "Mount",
    "Mountain",
    "Mountain",
    "Mountains",
    "Mountains",
    "Neck",
    "Orchard",
    "Oval",
    "Overpass",
    "Park",
    "Parks",
    "Parkway",
    "Parkways",
    "Pass",
    "Passage",
    "Path",
    "Pike",
    "Pine",
    "Pines",
    "Place",
    "Plain",
    "Plains",
    "Plains",
    "Plaza",
    "Plaza",
    "Point",
    "Points",
    "Port",
    "Port",
    "Ports",
    "Ports",
    "Prairie",
    "Prairie",
    "Radial",
    "Ramp",
    "Ranch",
    "Rapid",
    "Rapids",
    "Rest",
    "Ridge",
    "Ridges",
    "River",
    "Road",
    "Road",
    "Roads",
    "Roads",
    "Route",
    "Row",
    "Rue",
    "Run",
    "Shoal",
    "Shoals",
    "Shore",
    "Shores",
    "Skyway",
    "Spring",
    "Springs",
    "Springs",
    "Spur",
    "Spurs",
    "Square",
    "Square",
    "Squares",
    "Squares",
    "Station",
    "Station",
    "Stravenue",
    "Stravenue",
    "Stream",
    "Stream",
    "Street",
    "Street",
    "Streets",
    "Summit",
    "Summit",
    "Terrace",
    "Throughway",
    "Trace",
    "Track",
    "Trafficway",
    "Trail",
    "Trail",
    "Tunnel",
    "Tunnel",
    "Turnpike",
    "Turnpike",
    "Underpass",
    "Union",
    "Unions",
    "Valley",
    "Valleys",
    "Via",
    "Viaduct",
    "View",
    "Views",
    "Village",
    "Village",
    "Villages",
    "Ville",
    "Vista",
    "Vista",
    "Walk",
    "Walks",
    "Wall",
    "Way",
    "Ways",
    "Well",
    "Wells",
];
static STREETSUFFIXES_LEN: usize = STREETSUFFIXES.len();

static CITYPREFIXES: [&'static str; 7] = [
    "North",
    "East",
    "West",
    "South",
    "New",
    "Lake",
    "Port",
];
static CITYPREFIXES_LEN: usize = CITYPREFIXES.len();

static CITYSUFFIXES: [&'static str; 19] = [
    "town",
    "ton",
    "land",
    "ville",
    "berg",
    "burgh",
    "borough",
    "bury",
    "view",
    "port",
    "mouth",
    "stad",
    "furt",
    "chester",
    "mouth",
    "fort",
    "haven",
    "side",
    "shire"
];
static CITYSUFFIXES_LEN: usize = CITYSUFFIXES.len();

static STATES: [&'static str; 50] = [
    "AL",
    "AK",
    "AZ",
    "AR",
    "CA",
    "CO",
    "CT",
    "DE",
    "FL",
    "GA",
    "HI",
    "ID",
    "IL",
    "IN",
    "IA",
    "KS",
    "KY",
    "LA",
    "ME",
    "MD",
    "MA",
    "MI",
    "MN",
    "MS",
    "MO",
    "MT",
    "NE",
    "NV",
    "NH",
    "NJ",
    "NM",
    "NY",
    "NC",
    "ND",
    "OH",
    "OK",
    "OR",
    "PA",
    "RI",
    "SC",
    "SD",
    "TN",
    "TX",
    "UT",
    "VT",
    "VA",
    "WA",
    "WV",
    "WI",
    "WY"
];
static STATES_LEN: usize = STATES.len();

static SECONDARY_FORMATS: [&'static str; 2] = [
    "Suite",
    "Apt.",
];
static SECONDARY_FORMATS_LEN: usize = SECONDARY_FORMATS.len();