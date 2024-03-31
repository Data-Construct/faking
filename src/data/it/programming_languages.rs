use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn name() -> String {
	NAMES[seeder::gen_range(0..NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn creator() -> String {
	CREATORS[seeder::gen_range(0..CREATORS_LEN)].to_string()
}

static NAMES: [&'static str; 726] = [
	"A# .NET",
	"A# (Axiom)",
	"A-0 System",
	"A+",
	"A++",
	"ABAP",
	"ABC",
	"ABC ALGOL",
	"ABSET",
	"ABSYS",
	"ACC",
	"Accent",
	"Ace DASL (Distributed Application Specification Language)",
	"ACL2",
	"ACT-III",
	"Action!",
	"ActionScript",
	"Ada",
	"Adenine",
	"Agda",
	"Agilent VEE",
	"Agora",
	"AIMMS",
	"Aldor",
	"Alef",
	"ALF",
	"ALGOL 58",
	"ALGOL 60",
	"ALGOL 68",
	"ALGOL W",
	"Alice",
	"Alma-0",
	"AmbientTalk",
	"Amiga E",
	"AMOS",
	"AML (Arc Macro Language)",
	"AMPL",
	"AngelScript",
	"Apex (Salesforce.com)",
	"APL",
	"App Inventor for Androids visual block language",
	"AppleScript",
	"APT",
	"Arc",
	"ARexx",
	"Argus",
	"AspectJ",
	"Assembly language",
	"ATS",
	"Ateji PX",
	"AutoHotkey",
	"Autocoder",
	"AutoIt",
	"AutoLISP / Visual LISP",
	"Averest",
	"AWK",
	"Axum",
	"Active Server Pages",
	"B",
	"Babbage",
	"Ballerina",
	"Bash",
	"BASIC",
	"bc",
	"BCPL",
	"BeanShell",
	"Batch (Windows/Dos)",
	"Bertrand",
	"BETA",
	"Bistro",
	"BLISS",
	"Blockly",
	"BlooP",
	"Boo",
	"Boomerang",
	"Bourne shell (including bash and ksh)",
	"BPEL",
	"BrightScript",
	"Business Basic",
	"C",
	"C--",
	"C++ – ISO/IEC 14882",
	"C# – ISO/IEC 23270",
	"C/AL",
	"Caché ObjectScript",
	"C Shell (csh)",
	"Caml",
	"Calcpad",
	"Cayenne",
	"CDuce",
	"Cecil",
	"Cesil",
	"Céu",
	"Ceylon",
	"CFEngine",
	"Cg",
	"Ch",
	"Chapel",
	"Charity",
	"Charm",
	"CHILL",
	"CHIP-8",
	"chomski",
	"ChucK",
	"Cilk",
	"Citrine",
	"CL (IBM)",
	"Claire",
	"Clarion",
	"Clean",
	"Clipper",
	"CLIPS",
	"CLIST",
	"Clojure",
	"CLU",
	"CMS-2",
	"COBOL – ISO/IEC 1989",
	"CobolScript – COBOL Scripting language",
	"Cobra",
	"CoffeeScript",
	"ColdFusion",
	"COMAL",
	"Combined Programming Language (CPL)",
	"COMIT",
	"Common Intermediate Language (CIL)",
	"Common Lisp (also known as CL)",
	"COMPASS",
	"Component Pascal",
	"Constraint Handling Rules (CHR)",
	"COMTRAN",
	"Converge",
	"Cool",
	"Coq",
	"Coral 66",
	"CorVision",
	"COWSEL",
	"CPL",
	"Cryptol",
	"Crystal",
	"Csound",
	"CSP",
	"CUDA",
	"Cuneiform",
	"Curl",
	"Curry",
	"Cybil",
	"Cyclone",
	"Cython",
	"D",
	"DASL (Datapoints Advanced Systems Language)",
	"Dart",
	"Darwin",
	"DataFlex",
	"Datalog",
	"DATATRIEVE",
	"dBase",
	"dc",
	"DCL",
	"Delphi",
	"DinkC",
	"DIBOL",
	"Dog",
	"Draco",
	"DRAKON",
	"Dylan",
	"DYNAMO",
	"DAX (Data Analysis Expressions)",
	"E",
	"Ease",
	"Easy PL/I",
	"EASYTRIEVE PLUS",
	"eC",
	"ECMAScript",
	"Edinburgh IMP",
	"EGL",
	"Eiffel",
	"ELAN",
	"Elixir",
	"Elm",
	"Emacs Lisp",
	"Emerald",
	"Epigram",
	"EPL (Easy Programming Language)",
	"EPL (Eltron Programming Language)",
	"Erlang",
	"es",
	"Escher",
	"ESPOL",
	"Esterel",
	"Etoys",
	"Euclid",
	"Euler",
	"Euphoria",
	"EusLisp Robot Programming Language",
	"CMS EXEC (EXEC)",
	"EXEC 2",
	"Executable UML",
	"F",
	"F#",
	"F*",
	"Factor",
	"Falcon",
	"Fantom",
	"FAUST",
	"FFP",
	"Fjölnir",
	"FL",
	"Flavors",
	"Flex",
	"FlooP",
	"FLOW-MATIC",
	"FOCAL",
	"FOCUS",
	"FOIL",
	"FORMAC",
	"@Formula",
	"Forth",
	"Fortran – ISO/IEC 1539",
	"Fortress",
	"FoxBase",
	"FoxPro",
	"FP",
	"Franz Lisp",
	"Frege",
	"F-Script",
	"G",
	"Game Maker Language",
	"GameMonkey Script",
	"GAMS",
	"GAP",
	"G-code",
	"GDScript",
	"Genie",
	"GDL",
	"GJ",
	"GEORGE",
	"GLSL",
	"GNU E",
	"GM",
	"Go",
	"Go!",
	"GOAL",
	"Gödel",
	"Golo",
	"GOM (Good Old Mad)",
	"Google Apps Script",
	"Gosu",
	"GOTRAN",
	"GPSS",
	"GraphTalk",
	"GRASS",
	"Groovy",
	"Hack",
	"HAGGIS",
	"HAL/S",
	"Halide (programming language)",
	"Hamilton C shell",
	"Harbour",
	"Hartmann pipelines",
	"Haskell",
	"Haxe",
	"Hermes",
	"High Level Assembly",
	"HLSL",
	"Hop",
	"Hopscotch",
	"Hope",
	"Hugo",
	"Hume",
	"HyperTalk",
	"Hexa",
	"Io",
	"Icon (programming language)",
	"IBM Basic assembly language",
	"IBM BASICA",
	"IBM HAScript",
	"IBM Informix-4GL",
	"IBM RPG",
	"IDL",
	"J",
	"J#",
	"J++",
	"JADE",
	"JAL",
	"Janus (concurrent constraint programming language)",
	"Janus (time-reversible computing programming language)",
	"JASS",
	"Java",
	"JavaScript",
	"JCL",
	"JEAN",
	"Join Java",
	"JOSS",
	"Joule",
	"JOVIAL",
	"Joy",
	"JScript",
	"JScript .NET",
	"JavaFX Script",
	"jq",
	"Julia",
	"Jython",
	"K",
	"Kaleidoscope",
	"Kafe",
	"Karel",
	"Karel++",
	"KEE",
	"Kixtart",
	"Klerer-May System",
	"KIF",
	"Kojo",
	"Kotlin",
	"KRC",
	"KRL",
	"KRL (KUKA Robot Language)",
	"KRYPTON",
	"ksh",
	"Kodu",
	"L",
	"LabVIEW",
	"Ladder",
	"Lagoona",
	"LANSA",
	"Lasso",
	"LaTeX",
	"Lava",
	"LC-3",
	"Leda",
	"Legoscript",
	"LIL",
	"LilyPond",
	"Limbo",
	"Limnor",
	"LINC",
	"Lingo",
	"LIS",
	"LISA",
	"Lisaac",
	"Lisp – ISO/IEC 13816",
	"Lite-C",
	"Lithe",
	"Little b",
	"LLL",
	"Logo",
	"Logtalk",
	"LotusScript",
	"LPC",
	"LSE",
	"LSL",
	"LiveCode",
	"LiveScript",
	"Lua",
	"Lucid",
	"Lustre",
	"LYaPAS",
	"Lynx",
	"M (alternative name for the MUMPS programming language)",
	"M2000",
	"M2001",
	"M4",
	"M#",
	"Machine code",
	"MAD (Michigan Algorithm Decoder)",
	"MAD/I",
	"Magik",
	"Magma",
	"make",
	"Maude system",
	"Maple",
	"MAPPER (now part of BIS)",
	"MARK-IV (now VISION:BUILDER)",
	"Mary",
	"MASM Microsoft Assembly x86",
	"MATH-MATIC",
	"Mathematica",
	"MATLAB",
	"Maxima (see also Macsyma)",
	"Max (Max Msp – Graphical Programming Environment)",
	"MaxScript internal language 3D Studio Max",
	"Maya (MEL)",
	"M++",
	"MDL",
	"Mercury",
	"Mesa",
	"Metafont",
	"MetaQuotes Language (MQL4/MQL5)",
	"MHEG-5 (Interactive TV programming language)",
	"Microcode",
	"MicroScript",
	"MIIS",
	"Milk (programming language)",
	"MIMIC",
	"Mirah",
	"Miranda",
	"MIVA Script",
	"ML",
	"Model 204",
	"Modelica",
	"Modula",
	"Modula-2",
	"Modula-3",
	"Mohol",
	"MOO",
	"Mortran",
	"Mouse",
	"MPD",
	"Mathcad",
	"MSIL – deprecated name for CIL",
	"MSL",
	"MUMPS",
	"MuPAD",
	"Mutan",
	"Mystic Programming Language (MPL)",
	"NASM",
	"Napier88",
	"Neko",
	"Nemerle",
	"nesC",
	"NESL",
	"Net.Data",
	"NetLogo",
	"NetRexx",
	"NewLISP",
	"NEWP",
	"Newspeak",
	"NewtonScript",
	"NGL",
	"Nial",
	"Nice",
	"Nickle (NITIN)",
	"Nim",
	"NPL",
	"Not eXactly C (NXC)",
	"Not Quite C (NQC)",
	"NSIS",
	"Nu",
	"NWScript",
	"NXT-G",
	"o:XML",
	"Oak",
	"Oberon",
	"OBJ2",
	"Object Lisp",
	"ObjectLOGO",
	"Object REXX",
	"Object Pascal",
	"Objective-C",
	"Objective-J",
	"Obliq",
	"OCaml",
	"occam",
	"occam-π",
	"Octave",
	"OmniMark",
	"Onyx",
	"Opa",
	"Opal",
	"OpenCL",
	"OpenEdge ABL",
	"OPL",
	"OpenVera",
	"OPS5",
	"OptimJ",
	"Orc",
	"ORCA/Modula-2",
	"Oriel",
	"Orwell",
	"Oxygene",
	"Oz",
	"P",
	"P′′",
	"P#",
	"ParaSail (programming language)",
	"PARI/GP",
	"Pascal – ISO 7185",
	"PCASTL",
	"PCF",
	"PEARL",
	"PeopleCode",
	"Perl",
	"PDL",
	"Perl 6",
	"Pharo",
	"PHP",
	"Pico",
	"Picolisp",
	"Pict",
	"Pig (programming tool)",
	"Pike",
	"PIKT",
	"PILOT",
	"Pipelines",
	"Pizza",
	"PL-11",
	"PL/0",
	"PL/B",
	"PL/C",
	"PL/I – ISO 6160",
	"PL/M",
	"PL/P",
	"PL/SQL",
	"PL360",
	"PLANC",
	"munoz",
	"Plankalkül",
	"Planner",
	"PLEX",
	"PLEXIL",
	"Plus",
	"Pony",
	"POP-11",
	"POP-2",
	"PostScript",
	"PortablE",
	"Powerhouse",
	"PowerBuilder – 4GL GUI application generator from Sybase",
	"PowerShell",
	"PPL",
	"Processing",
	"Processing.js",
	"Prograph",
	"PROIV",
	"Prolog",
	"PROMAL",
	"Promela",
	"PROSE modeling language",
	"PROTEL",
	"ProvideX",
	"Pro*C",
	"Pure",
	"PureBasic",
	"Pure Data",
	"Python",
	"Q (equational programming language)",
	"Q (programming language from Kx Systems)",
	"Q# (Microsoft programming language)",
	"Qalb",
	"QtScript",
	"QuakeC",
	"QPL",
	"R",
	"R++",
	"Racket",
	"RAPID",
	"Rapira",
	"Ratfiv",
	"Ratfor",
	"rc",
	"Reason",
	"REBOL",
	"Red",
	"Redcode",
	"REFAL",
	"Reia",
	"REXX",
	"Ring",
	"Rlab",
	"ROOP",
	"RPG",
	"RPL",
	"RSL",
	"RTL/2",
	"Ruby",
	"RuneScript",
	"Rust",
	"S",
	"S2",
	"S3",
	"S-Lang",
	"S-PLUS",
	"SA-C",
	"SabreTalk",
	"SAIL",
	"SALSA",
	"SAM76",
	"SAS",
	"SASL",
	"Sather",
	"Sawzall",
	"SBL",
	"Scala",
	"Scheme",
	"Scilab",
	"Script.NET",
	"Sed",
	"Seed7",
	"Self",
	"SenseTalk",
	"SequenceL",
	"Serpent",
	"SETL",
	"SIMPOL",
	"SIGNAL",
	"SiMPLE",
	"SIMSCRIPT",
	"Simula",
	"Simulink",
	"Singularity",
	"SISAL",
	"SLIP",
	"SMALL",
	"Scratch",
	"Smalltalk",
	"SML",
	"Strongtalk",
	"Snap!",
	"SNOBOL(SPITBOL)",
	"Snowball",
	"SOL",
	"Solidity",
	"SOPHAEROS",
	"SPARK",
	"Speedcode",
	"SPIN",
	"SP/k",
	"SPS",
	"SQR",
	"Squeak",
	"Squirrel",
	"SR",
	"S/SL",
	"Stackless Python",
	"Starlogo",
	"Strand",
	"Stata",
	"Stateflow",
	"Subtext",
	"SuperCollider",
	"SuperTalk",
	"Swift (Apple programming language)",
	"Swift (parallel scripting language)",
	"SYMPL",
	"SyncCharts",
	"SystemVerilog",
	"T",
	"TACL",
	"TACPOL",
	"TADS",
	"TAL",
	"Tcl",
	"Tea",
	"TECO",
	"TELCOMP",
	"TeX",
	"TEX",
	"TIE",
	"Timber",
	"TMG, compiler-compiler",
	"Tom",
	"TOM",
	"Toi",
	"Topspeed",
	"TPU",
	"Trac",
	"TTM",
	"T-SQL",
	"Transcript",
	"TTCN",
	"Turing",
	"TUTOR",
	"TXL",
	"TypeScript",
	"Tynker",
	"Ubercode",
	"UCSD Pascal",
	"Umple",
	"Unicon",
	"Uniface",
	"UNITY",
	"UniVerse Basic",
	"Unix shell",
	"UnrealScript",
	"Vala",
	"Verilog",
	"Viper",
	"Visual Basic",
	"Visual Basic .NET",
	"Visual DataFlex",
	"Visual DialogScript",
	"Visual Fortran",
	"Visual FoxPro",
	"Visual J++",
	"Visual J#",
	"Visual LISP",
	"Visual Objects",
	"Visual Prolog",
	"VSXu",
	"VVML (Verification Validation Mark-up Language)",
	"vvvv",
	"WATFIV, WATFOR",
	"WebDNA",
	"WebQL",
	"Whiley",
	"Winbatch",
	"Wolfram Language",
	"Wyvern",
	"X++",
	"X10",
	"XBL",
	"XC (exploits XMOS architecture)",
	"xHarbour",
	"XL",
	"Xojo",
	"XOTcl",
	"XPL",
	"XPL0",
	"XQuery",
	"XSB",
	"XSharp",
	"XSLT – see XPath",
	"Xtend",
	"Yorick",
	"YQL",
	"Yoix",
	"Z",
	"Z notation",
	"Zap",
	"Zebra, ZPL, ZPL2",
	"Zeno",
	"ZetaLisp",
	"ZIL",
	"ZOPL",
	"Zsh",
	"ZPL",
	"Z++",
];
static NAMES_LEN: usize = NAMES.len();

static CREATORS: [&'static str; 25] = [
	"John Backus",
	"Friedrich L. Bauer",
	"Gilad Bracha",
	"Walter Bright",
	"Alain Colmerauer",
	"Ole-Johan Dahl",
	"Brendan Eich",
	"James Gosling",
	"Anders Hejlsberg",
	"Rich Hickey",
	"Roberto Ierusalimschy",
	"Alan Kay",
	"Dan Ingalls",
	"Chris Lattner",
	"Yukihiro Matsumoto",
	"John McCarthy",
	"Martin Odersky",
	"Dennis Ritchie",
	"Guido van Rossum",
	"Guy L. Steele, Jr.",
	"Bjarne Stroustrup",
	"Don Syme",
	"Ken Thompson",
	"Larry Wall",
	"Philip Wadler",
];
static CREATORS_LEN: usize = CREATORS.len();
