use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn random_vehicle_make() -> String {
	let make = MAKES[seeder::gen_range(0..MAKES_LEN)];

	match make {
		"BMW" => concat_string!(make, " ", BMW_MODELS[seeder::gen_range(0..BMW_MODELS_LEN)].to_string()),
		"Audi" => concat_string!(make, " ", AUDI_MODELS[seeder::gen_range(0..AUDI_MODELS_LEN)].to_string()),
		"Toyota" => concat_string!(make, " ", TOYOTA_MODELS[seeder::gen_range(0..TOYOTA_MODELS_LEN)].to_string()),
		"Chevrolet" => concat_string!(make, " ", CHEVROLET_MODELS[seeder::gen_range(0..CHEVROLET_MODELS_LEN)].to_string()),
		"Ford" => concat_string!(make, " ", FORD_MODELS[seeder::gen_range(0..FORD_MODELS_LEN)].to_string()),
		"Dodge" => concat_string!(make, " ", DODGE_MODELS[seeder::gen_range(0..DODGE_MODELS_LEN)].to_string()),
		"Lincoln" => concat_string!(make, " ", LINCOLN_MODELS[seeder::gen_range(0..LINCOLN_MODELS_LEN)].to_string()),
		"Buick" => concat_string!(make, " ", BUICK_MODELS[seeder::gen_range(0..BUICK_MODELS_LEN)].to_string()),
		"Honda" => concat_string!(make, " ", HONDA_MODELS[seeder::gen_range(0..HONDA_MODELS_LEN)].to_string()),
		"Nissan" => concat_string!(make, " ", NISSAN_MODELS[seeder::gen_range(0..NISSAN_MODELS_LEN)].to_string()),
		"Mercedes-Benz" => concat_string!(make, " ", MERCEDES_MODELS[seeder::gen_range(0..MERCEDES_MODELS_LEN)].to_string()),
		"Lexus" => concat_string!(make, " ", LEXUS_MODELS[seeder::gen_range(0..LEXUS_MODELS_LEN)].to_string()),
		"Tesla" => concat_string!(make, " ", TESLA_MODELS[seeder::gen_range(0..TESLA_MODELS_LEN)].to_string()),
		"Mazda" => concat_string!(make, " ", MAZDA_MODELS[seeder::gen_range(0..MAZDA_MODELS_LEN)].to_string()),
		"Hyundai" => concat_string!(make, " ", HYUNDAI_MODELS[seeder::gen_range(0..HYUNDAI_MODELS_LEN)].to_string()),
		"Cadillac" => concat_string!(make, " ", CADILLAC_MODELS[seeder::gen_range(0..CADILLAC_MODELS_LEN)].to_string()),
		_ => "".to_string(),
	}
}

#[wasm_bindgen]
pub fn random_vehicle_style() -> String {
	STYLES[seeder::gen_range(0..STYLES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_vehicle_drive_type() -> String {
	DRIVE_TYPES[seeder::gen_range(0..DRIVE_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_vehicle_fuel_types() -> String {
	FUEL_TYPES[seeder::gen_range(0..FUEL_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_vehicle_car_types() -> String {
	CAR_TYPES[seeder::gen_range(0..CAR_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_vehicle_car_options() -> String {
	CAR_OPTIONS[seeder::gen_range(0..CAR_OPTIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn random_vehicle_car_specs() -> String {
	CAR_SPECS[seeder::gen_range(0..CAR_SPECS_LEN)].to_string()
}

static STYLES: [&'static str; 4] = [
    "XL",
    "L",
    "ESi",
    "XLE"
];
static STYLES_LEN: usize = STYLES.len();

static DRIVE_TYPES: [&'static str; 5] = [
    "4x2/2-wheel drive",
    "4x4/4-wheel drive",
    "AWD",
    "FWD",
    "RWD",
];
static DRIVE_TYPES_LEN: usize = DRIVE_TYPES.len();

static FUEL_TYPES: [&'static str; 7] = [
    "Compressed Natural Gas",
    "Diesel",
    "E-85/Gasoline",
    "Electric",
    "Gasoline",
    "Gasoline Hybrid",
    "Ethanol",
];
static FUEL_TYPES_LEN: usize = FUEL_TYPES.len();

static CAR_TYPES: [&'static str; 12] = [
    "Cargo Van",
    "Convertible",
    "Coupe",
    "Crew Cab Pickup",
    "Extended Cab Pickup",
    "Hatchback",
    "Minivan",
    "Passenger Van",
    "Regular Cab Pickup",
    "SUV",
    "Sedan",
    "Wagon",
];
static CAR_TYPES_LEN: usize = CAR_TYPES.len();

static CAR_OPTIONS: [&'static str; 34] = [
    "'A/C: Front'",
    "'Airbag: Driver'",
    "AM/FM Stereo",
    "'A/C: Rear'",
    "'Airbag: Passenger'",
    "Cassette Player",
    "Cruise Control",
    "'Airbag: Side'",
    "CD (Single Disc)",
    "Navigation",
    "Alarm",
    "CD (Multi Disc)",
    "Power Locks",
    "Antilock Brakes",
    "MP3 (Single Disc)",
    "Power Steering",
    "Fog Lights",
    "MP3 (Multi Disc)",
    "Keyless Entry",
    "Premium Sound",
    "Integrated Phone",
    "DVD System",
    "Bucket Seats",
    "Power Windows",
    "Alloy Wheels",
    "Leather Interior",
    "Rear Window Defroster",
    "Moonroof/Sunroof",
    "Memory Seats",
    "Rear Window Wiper",
    "Third Row Seats",
    "Power Seats",
    "Tinted Glass",
    "Tow Package",
];
static CAR_OPTIONS_LEN: usize = CAR_OPTIONS.len();

static CAR_SPECS: [&'static str; 189] = [
    "'1.8L DOHC 16-valve I4 engine -inc: engine cover'",
    "'Engine mounts -inc: (2) solid, (1) liquid-filled'",
    "Front wheel drive",
    "Battery saver",
    "Independent strut front suspension w/stabilizer bar",
    "Torsion beam rear suspension w/stabilizer bar",
    "Electric speed-sensitive variable-assist pwr steering",
    "Pwr front vented disc/rear drum brakes",
    "Compact spare tire",
    "Body color front/rear bumpers",
    "Multi-reflector halogen headlamps",
    "Body color folding remote-controlled pwr mirrors",
    "Variable intermittent windshield wipers w/mist function",
    "Intermittent rear wiper w/washer",
    "Body color door handles",
    "Roof mounted antenna",
    "'Reclining front bucket seats -inc: active head restraints, double-thickness foam in front seats'",
    "60/40 split fold-down rear seat w/outboard adjustable headrests",
    "Dual front & rear cup holders",
    "Tilt steering column",
    "'Silver accent IP trim finisher -inc: silver shifter finisher'",
    "Tachometer",
    "Fasten seat belt warning light/chime",
    "Pwr windows",
    "Remote fuel lid release",
    "Immobilizer system",
    "Pwr rear liftgate release",
    "Air conditioning w/in-cabin microfilter",
    "Rear window defroster w/timer",
    "12V pwr outlet",
    "Silver finish interior door handles",
    "Driver & front passenger map pockets",
    "Rear passenger map pockets",
    "Front & rear passenger folding assist grips",
    "Carpeted floor & cargo area",
    "Cargo area lamp",
    "'Anti-lock brake system (ABS) -inc: electronic brake force distribution (EBD), brake assist'",
    "Energy absorbing front/rear bumpers",
    "Steel side-door impact beams",
    "'Zone body construction -inc: front/rear crumple zones, hood deformation point'",
    "Dual-stage front airbags w/occupant classification system",
    "Front side-impact airbags",
    "Front & rear side curtain airbags",
    "3-point ELR driver seat belt w/pretensioner & load limiter",
    "3-point ELR/ALR front passenger seat belt w/pretensioner & load limiter",
    "3-point ELR/ALR rear seat belts at all positions",
    "Child safety rear door locks",
    "Rear child seat tether anchors (LATCH)",
    "Tire pressure monitoring system (TPMS)",
    "Energy absorbing steering column",
    "'4.6L DOHC 32-valve V8 engine -inc: DI & SFI dual fuel injection, dual variable valve timing w/intelligence & electronically controlled intake (VVT-iE), aluminum block & heads'",
    "Vibration-dampening liquid-filled engine mounts",
    "Electronic throttle control system w/intelligence (ETCS-i)",
    "Acoustic control induction system (ACIS)",
    "'8-speed automatic transmission -inc: intelligence (ECT-i), gated shifter, sequential sport-shift mode'",
    "Full-time all-wheel drive",
    "Front/rear aluminum multi-link double joint suspension w/coil springs",
    "Front/rear stabilizer bars",
    "Electric pwr rack & pinion steering (EPS)",
    "'4-wheel ventilated pwr disc brakes -inc: brake override system'",
    "Dual chrome exhaust tips",
    "Tool kit",
    "P235/50R18 all-season tires",
    "Full-size spare tire w/aluminum alloy wheel",
    "Scratch-resistant paint clearcoating",
    "'Pwr tilt/slide moonroof -inc: 1-touch open/close'",
    "1-piece chrome window surround",
    "'Xenon high-intensity discharge (HID) headlamps -inc: adaptive front lighting system, delayed auto-off'",
    "Integrated fog lamps",
    "'LED lights -inc: brake lamps, tail lamps, license plate'",
    "'Electrochromic pwr folding heated mirrors w/memory -inc: puddle lamps, integrated turn signals, auto reverse tilt-down'",
    "Acoustic glass windshield",
    "Water-repellent windshield & front door glass",
    "Laminated side window glass",
    "Rain-sensing wipers",
    "'XM satellite radio receiver -inc: 90 day trial subscription'",
    "'Rear bench seat -inc: (3) adjustable headrests'",
    "Center console",
    "Optitron electroluminescent instrumentation",
    "'Multi-info display -inc: driving range, average MPG, current MPG, average speed, outside temp, elapsed time, maintenance & diagnostic messages'",
    "Eco drive indicator",
    "'Pwr windows -inc: 1-touch open/close'",
    "HomeLink universal transceiver",
    "'Dual-zone automatic climate control system -inc: smog sensor, auto recirculation, clear air filter, pollen filter'",
    "Rear-window defogger w/auto-off timer",
    "\"(2) aux 12V pwr outlets -inc: (1) in center console, (1) w/cigarette lighter\"",
    "'Grain-matched wood trim -inc: center console, dash, door panels'",
    "Electrochromic rearview mirror",
    "Foldable front door storage pockets",
    "Dual front illuminated visor vanity mirrors",
    "Front/rear spot-lamp illumination",
    "4-wheel/4-channel anti-lock brake system (ABS)",
    "Electronic control braking (ECB)",
    "'Electronic brakeforce distribution (EBD) w/brake assist (BA) -inc: Smart stop technology'",
    "Electronic parking brake",
    "'Vehicle dynamics integrated management (VDIM) system -inc: vehicle stability control (VSC), traction control (TRAC)'",
    "Front/rear crumple zones",
    "Daytime running lights (DRL)",
    "Side-impact door beams",
    "'Dual front 2-stage airbags -inc: passenger occupant classification system w/twin-chamber airbag'",
    "Front/rear side curtain airbags",
    "Dual front knee airbags",
    "Back-up camera",
    "'All-position 3-point seat belts -inc: outboard pretensioners & force limiters, dual front pwr shoulder height adjusters, rear outboard emergency auto locking retractors, driver emergency locking retractor'",
    "Child restraint seat anchors for outboard positions",
    "Rear door child safety locks",
    "Direct-type tire pressure monitor system",
    "Impact-dissipating upper interior trim",
    "Collapsible steering column",
    "Emergency interior trunk release",
    "First aid kit",
    "6.1L SRT V8 \"Hemi\" engine",
    "3.73 axle ratio",
    "Quadra-Trac active on demand 4WD system",
    "200mm front axle",
    "Dana 44/226mm rear axle",
    "625-amp maintenance-free battery",
    "160-amp alternator",
    "Tip start system",
    "Pwr accessory delay",
    "Trailer tow wiring harness",
    "High performance suspension",
    "Pwr steering cooler",
    "Pwr rack & pinion performance tuned steering",
    "Anti-lock 4-wheel performance disc brakes",
    "Brake assist",
    "Dual bright exhaust tips",
    "Run flat tires",
    "20\" x 9.0\" front & 20\" x 10.0\" rear aluminum wheels",
    "Monotone paint",
    "Black roof molding",
    "Rear body-color spoiler",
    "Body color grille",
    "Chrome bodyside molding",
    "Black windshield molding",
    "Body color fascias w/bright insert",
    "Body color sill extension",
    "Fog lamps",
    "Front door tinted glass",
    "'\"Flipper\" liftgate glass'",
    "Rear window wiper/washer",
    "Body color front license plate brow",
    "Body color door handles",
    "6.5\" touch screen display",
    "Fixed long mast antenna",
    "Pwr 8-way driver seat w/4-way front passenger seat",
    "60/40 folding rear seat",
    "Full-length floor console",
    "Luxury front & rear floor mats w/logo",
    "Floor carpeting",
    "Tilt/telescoping steering column",
    "Leather-wrapped steering wheel w/audio controls",
    "Instrument cluster w/tachometer",
    "Vehicle info center",
    "Traveler/mini trip computer",
    "Pwr front windows w/(1) touch up/down feature",
    "Speed control",
    "Sentry Key theft deterrent system",
    "Security alarm",
    "Bright pedals",
    "Rear window defroster",
    "Locking glove box",
    "Highline door trim panel",
    "Cloth covered headliner",
    "Overhead console",
    "Dual illuminated visor vanity mirrors",
    "Universal garage door opener",
    "Passenger assist handles",
    "Deluxe insulation group",
    "Cargo compartment lamp",
    "Glove box lamp",
    "Rear reading & courtesy lamps",
    "Illuminated entry",
    "Leather-wrapped shift knob",
    "Leather-wrapped parking brake handle",
    "Carpeted cargo area",
    "Trim-panel-mounted storage net",
    "Cargo-area tie down loops",
    "Cargo compartment cover",
    "Reversible/waterproof cargo storage",
    "Driver & front passenger advanced multistage airbags w/occupant sensors",
    "Supplemental side curtain air bags",
    "Enhanced accident response system unlocks the doors, shuts off the fuel pump and turns on interior lights after airbag deploys",
    "3-point rear center seat belts",
    "Child seat upper tether anchorages",
    "LATCH-ready child seat anchor system",
    "Child safety rear door locks",
    "Dual note horn",
    "Tire pressure monitoring display",
];
static CAR_SPECS_LEN: usize = CAR_SPECS.len();

static MAKES: [&'static str; 17] = [
	"BMW",
	"Audi",
	"Toyota",
	"Chevrolet",
	"Ford",
	"Dodge",
	"Lincoln",
	"Buick",
	"Honda",
	"Nissan",
	"Mercedes-Benz",
	"Lexus",
	"Tesla",
	"Mazda",
	"Hyundai",
    "Cadillac",
    "Acura"
];
static MAKES_LEN: usize = MAKES.len();

static BMW_MODELS: [&'static str; 6] = [
	"328i",
	"M3",
	"M5",
	"X1",
	"X3",
	"X5",
];
static BMW_MODELS_LEN: usize = BMW_MODELS.len();

static AUDI_MODELS: [&'static str; 4] = [
	"A4",
	"A6",
	"A7",
	"A8",
];
static AUDI_MODELS_LEN: usize = AUDI_MODELS.len();

static TOYOTA_MODELS: [&'static str; 8] = [
	"Corolla",
	"Camry",
	"Mirai",
	"Rav4",
	"Venza",
	"Highlander",
	"Sienna",
	"Tundra",
];
static TOYOTA_MODELS_LEN: usize = TOYOTA_MODELS.len();

static CHEVROLET_MODELS: [&'static str; 6] = [
	"Bolt",
    "Tahoe",
    "Silverado",
    "Malibu",
    "Camaro",
    "Corvette",
];
static CHEVROLET_MODELS_LEN: usize = CHEVROLET_MODELS.len();

static FORD_MODELS: [&'static str; 6] = [
	"Mustang",
    "Escape",
    "Edge",
    "F-150",
    "Ranger",
    "Explorer",
];
static FORD_MODELS_LEN: usize = FORD_MODELS.len();

static DODGE_MODELS: [&'static str; 4] = [
	"Hornet",
    "Challenger",
    "Charger",
    "Durango",
];
static DODGE_MODELS_LEN: usize = DODGE_MODELS.len();

static LINCOLN_MODELS: [&'static str; 4] = [
	"Navigator",
    "Aviator",
    "Nautilus",
    "Corsair",
];
static LINCOLN_MODELS_LEN: usize = LINCOLN_MODELS.len();

static BUICK_MODELS: [&'static str; 4] = [
	"Envista",
    "Encore",
    "Envision",
    "Enclave",
];
static BUICK_MODELS_LEN: usize = BUICK_MODELS.len();

static HONDA_MODELS: [&'static str; 10] = [
	"Prologue",
    "Civic",
    "Type R",
    "Accord",
	"HR-V",
    "CR-V",
    "Passport",
    "Pilot",
    "Odyssey",
    "Ridgeline",
];
static HONDA_MODELS_LEN: usize = HONDA_MODELS.len();

static NISSAN_MODELS: [&'static str; 12] = [
	"Kicks",
    "Qashqai",
	"Rogue",
    "Murano",
	"Pathfinder",
    "Versa",
	"Sentra",
    "Altima",
	"Maxima",
    "GT-R",
	"Leaf",
    "Frontier",
];
static NISSAN_MODELS_LEN: usize = NISSAN_MODELS.len();

static MERCEDES_MODELS: [&'static str; 18] = [
	"EQB SUV",
    "EQE Sedan",
    "EQE SUV",
	"EQS SUV",
    "EQS Sedan",
    "GLA SUV",
    "GLB SUV",
    "GLC SUV",
    "GLE SUV",
    "GLS SUV",
    "A-Class Hatch",
    "A-Class Sedan",
    "C-Class Sedan",
    "E-Class Sedan",
    "S-Class Sedan",
    "C-Class Coupe",
    "E-Class Coupe",
    "SL Roadster",
];
static MERCEDES_MODELS_LEN: usize = MERCEDES_MODELS.len();

static LEXUS_MODELS: [&'static str; 12] = [
	"IS",
	"RC",
	"ES",
	"LC",
	"LS",
	"UXh",
	"NX",
	"RZ",
	"RX",
	"TX",
	"GX",
	"LX",
];
static LEXUS_MODELS_LEN: usize = LEXUS_MODELS.len();

static TESLA_MODELS: [&'static str; 5] = [
	"Model S",
	"Model 3",
	"Model X",
	"Model Y",
	"Cybertruck",
];
static TESLA_MODELS_LEN: usize = TESLA_MODELS.len();

static MAZDA_MODELS: [&'static str; 6] = [
	"CX-70",
	"CX-90",
	"CX-5",
	"CX-50",
	"CX-30",
	"3",
];
static MAZDA_MODELS_LEN: usize = MAZDA_MODELS.len();

static HYUNDAI_MODELS: [&'static str; 7] = [
	"Elantra",
	"Ioniq",
	"Sonata",
	"Tuscon",
	"Santa Fe",
	"Santa Cruz",
	"Kona",
];
static HYUNDAI_MODELS_LEN: usize = HYUNDAI_MODELS.len();

static CADILLAC_MODELS: [&'static str; 6] = [
	"CT4",
	"CT5",
	"XT4",
	"XT5",
	"XT6",
	"Escalade",
];
static CADILLAC_MODELS_LEN: usize = CADILLAC_MODELS.len();

static ACURA_MODELS: [&'static str; 5] = [
	"ZDX",
	"MDX",
	"RDX",
	"Integra",
	"TLX",
];
static ACURA_MODELS_LEN: usize = ACURA_MODELS.len();