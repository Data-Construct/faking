use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lorem_ipsum_word() -> String {
	LOREM_IPSUM_WORDS[seeder::gen_range(0..LOREM_IPSUM_WORDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lorem_ipsum_words(amount: i16) -> String {
	let mut lorem_text = String::from("");

	for n in 0..amount {
		lorem_text.push_str(&lorem_ipsum_word());
		lorem_text.push(' ');
	}

	lorem_text.to_string()
}

#[wasm_bindgen]
pub fn lorem_ipsum_sentence() -> String {
	let mut lorem_text = String::from("");

	let word_amount = seeder::gen_range(10..31);
	for n in 0..word_amount {
		if n == 0 {
			lorem_text.push_str(&capitalize_first(&lorem_ipsum_word()));
			lorem_text.push(' ');
		} else if n >= word_amount - 1 {
			lorem_text.push_str(&lorem_ipsum_word());
			lorem_text.push('.');
		} else {
			lorem_text.push_str(&lorem_ipsum_word());
			lorem_text.push(' ');
		}
	}

	lorem_text.to_string()
}

#[wasm_bindgen]
pub fn lorem_ipsum_sentence_minmax(min_words: i16, max_words: i16) -> String {
	let mut lorem_text = String::from("");

	let word_amount = seeder::gen_range(min_words..max_words);
	for n in 0..word_amount {
		if n == 0 {
			lorem_text.push_str(&capitalize_first(&lorem_ipsum_word()));
			lorem_text.push(' ');
		} else if n >= word_amount - 1 {
			lorem_text.push_str(&lorem_ipsum_word());
			lorem_text.push('.');
		} else {
			lorem_text.push_str(&lorem_ipsum_word());
			lorem_text.push(' ');
		}
	}

	lorem_text.to_string()
}
#[wasm_bindgen]
pub fn lorem_ipsum_sentences(amount: i16, min_words: i16, max_words: i16) -> String {
	let mut lorem_text = String::from("");

	//let word_amount = seeder::gen_range(min_words..max_words);
	for n in 0..amount {
		if (n >= amount - 1) {
			lorem_text.push_str(&lorem_ipsum_sentence_minmax(min_words, max_words));
		} else {
			lorem_text.push_str(&lorem_ipsum_sentence_minmax(min_words, max_words));
			lorem_text.push(' ');
		}
	}

	lorem_text.to_string()
}
#[wasm_bindgen]
pub fn lorem_ipsum_paragraph() -> String {
	let mut lorem_text = String::from("");

	for n in 0..4 {
		if n >= 3 {
			lorem_text.push_str(&lorem_ipsum_sentence());
		} else {
			lorem_text.push_str(&lorem_ipsum_sentence());
			lorem_text.push(' ');
		}
	}

	lorem_text.to_string()
}

#[wasm_bindgen]
pub fn lorem_ipsum_paragraph_minmax(
	min_sentence: i16,
	max_sentence: i16,
	min_words: i16,
	max_words: i16,
) -> String {
	let mut lorem_text = String::from("");
	let sentence_amount = seeder::gen_range(min_sentence..max_sentence);
	for n in 0..sentence_amount {
		if n >= sentence_amount {
			lorem_text.push_str(&lorem_ipsum_sentence_minmax(min_words, max_words));
		} else {
			lorem_text.push_str(&lorem_ipsum_sentence_minmax(min_words, max_words));
			lorem_text.push(' ');
		}
	}

	lorem_text.to_string()
}

#[wasm_bindgen]
pub fn lorem_ipsum_paragraphs(amount: i16) -> String {
	let mut lorem_text = String::from("");

	//Should i return an error if 0 or ??
	if amount < 1 {
		"".to_string();
	}

	for n in 0..amount {
		if n < amount - 1 {
			let paragraph = lorem_ipsum_paragraph() + "\n\n";
			lorem_text.push_str(&paragraph);
		} else {
			lorem_text.push_str(&lorem_ipsum_paragraph());
		}
	}

	lorem_text.to_string()
}

fn capitalize_first(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
	}
}

static LOREM_IPSUM_WORDS: [&'static str; 69] = [
	"lorem",
	"ipsum",
	"dolor",
	"sit",
	"amet",
	"consectetur",
	"adipiscing",
	"elit",
	"sed",
	"do",
	"eiusmod",
	"tempor",
	"incididunt",
	"ut",
	"labore",
	"et",
	"dolore",
	"magna",
	"aliqua",
	"ut",
	"enim",
	"ad",
	"minim",
	"veniam",
	"quis",
	"nostrud",
	"exercitation",
	"ullamco",
	"laboris",
	"nisi",
	"ut",
	"aliquip",
	"ex",
	"ea",
	"commodo",
	"consequat",
	"duis",
	"aute",
	"irure",
	"dolor",
	"in",
	"reprehenderit",
	"in",
	"voluptate",
	"velit",
	"esse",
	"cillum",
	"dolore",
	"eu",
	"fugiat",
	"nulla",
	"pariatur",
	"Excepteur",
	"sint",
	"occaecat",
	"cupidatat",
	"non",
	"proident",
	"sunt",
	"in",
	"culpa",
	"qui",
	"officia",
	"deserunt",
	"mollit",
	"anim",
	"id",
	"est",
	"laborum",
];
static LOREM_IPSUM_WORDS_LEN: usize = LOREM_IPSUM_WORDS.len();
