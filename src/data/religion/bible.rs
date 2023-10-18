use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bible_characters() -> String {
	let mut rng = rand::thread_rng();
	CHARACTERS[rng.gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn bible_locations() -> String {
	let mut rng = rand::thread_rng();
	LOCATIONS[rng.gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn bible_quotes() -> String {
	let mut rng = rand::thread_rng();
	QUOTES[rng.gen_range(0..QUOTES_LEN)].to_string()
}

static CHARACTERS: [&'static str; 48] = [
	"Adam",
	"Eve ",
	"Cain",
	"Abel",
	"Noah",
	"Abraham",
	"Isaac",
	"Jacob",
	"Esau",
	"Rebekah",
	"Moses",
	"Aaron",
	"Joseph",
	"Joshua",
	"Caleb",
	"Ruth",
	"Samson",
	"Gideon",
	"Esther",
	"Nehemiah",
	"Solomon",
	"Abimelech",
	"Jeremiah",
	"Isaiah",
	"Daniel",
	"Zechariah",
	"Jesus",
	"Apostle Paul",
	"David",
	"Goliath",
	"Samuel",
	"Martha",
	"Mary",
	"Elizabeth",
	"John the Baptist",
	"Peter",
	"James",
	"John",
	"Elijah",
	"Elisha",
	"Lazarus",
	"Cornelius",
	"Chloe",
	"Eunice",
	"Lois",
	"Timothy",
	"Titus",
	"Bathemaeus",
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static LOCATIONS: [&'static str; 23] = [
	"Egypt",
	"Nile",
	"Red Sea",
	"Niniveh",
	"Malta",
	"Greece",
	"Troas",
	"Philippi",
	"Ephesus",
	"Corinth",
	"Macedonia",
	"Thessalonica",
	"Colossae",
	"Achaia",
	"Samaria",
	"Galatia",
	"Judea",
	"Damascus",
	"Rome",
	"Galilee",
	"Syria",
	"Babylon",
	"Jerusalem",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 11] = [
    "I am the way and the truth and the life. No one comes to the Father except through me.",
    "But seek first his kingdom and his righteousness, and all these things will be given to you as well.",
    "In the same way, let your light shine before others, that they may see your good deeds and glorify your Father in heaven.",
    "Ask and it will be given to you; seek and you will find; knock and the door will be opened to you. For everyone who asks receives; the one who seeks finds; and to the one who knocks, the door will be opened.",
    "'Love the Lord your God with all your heart and with all your soul and with all your mind. This is the first and greatest commandment. And the second is like it: Love your neighbor as yourself. All the Law and the Prophets hang on these two commandments.'",
    "Whatever you do, work at it with all your heart.",
    "For when I am powerless, it is then that I am strong.",
    "Love is patient, love is kind, and is not jealous; love does not brag and is not arrogant, does not act unbecomingly; it does not seek its own [will], is not provoked, does not take into account a wrong suffered, does not rejoice in unrighteousness, but rejoices with the truth; bears all things, believes all things, hopes all things, endures all things.",
    "There is no longer Jew or Greek, there is no longer slave or free, there is no longer male and female; for all of you are one in Christ Jesus.",
    "I am not saying this because I am in need, for I have learned to be content whatever the circumstances. I know what it is to be in need, and I know what it is to have plenty. I have learned the secret of being content in any and every situation, whether well fed or hungry, whether living in plenty or in want. I can do everything through him who gives me strength.",
    "Let us not become weary in doing good, for at the proper time we will reap a harvest if we do not give up",
];
static QUOTES_LEN: usize = QUOTES.len();
