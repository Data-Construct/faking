use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greek_philosopher_names() -> String {
	NAMES[seeder::gen_range(0..NAMES_LEN)].to_string()
}

// Left in for testing against seeder
// #[wasm_bindgen]
// pub fn greek_philosopher_quotes_z() -> String {
//
// 	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
// }

#[wasm_bindgen]
pub fn greek_philosopher_quotes() -> String {
	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

static NAMES: [&'static str; 22] = [
	"Plato",
	"Aristotle",
	"Pythagoras",
	"Heraclitus",
	"Parmenides",
	"Democritus",
	"Zeno of Elea",
	"Epicurus",
	"Anaxagoras",
	"Diogenes",
	"Antisthenes",
	"Gorgias",
	"Hippocrates",
	"Plutarch",
	"Proclus",
	"Chrysippus",
	"Solon",
	"Archimedes",
	"Thucydides",
	"Arcesilaus",
	"Posidonius",
	"Galen",
];
static NAMES_LEN: usize = NAMES.len();

static QUOTES: [&'static str; 20] = [
	"Quality is not an act, it is a habit.",
	"Only the educated are free.",
	"Control thy passions lest they take vengence on thee.",
	"Love is composed of a single soul inhabiting two bodies.",
	"Rhetoric is the art of ruling the minds of men.",
	"The unexamined life is not worth living.",
	"There was never a genius without a tincture of madness.",
	"Dignity does not consist in possessing honors, but in deserving them.",
	"Most people would rather give than get affection.",
	"Beware the barrenness of a busy life.",
	"The secret to humor is surprise.",
	"Pleasure in the job puts perfection in the work.",
	"Philosophy is the highest music.",
	"The virtue of justice consists in moderation, as regulated by wisdom.",
	"Know how to listen, and you will profit even from those who talk badly.",
	"A few vices are sufficient to darken many virtues.",
	"It is impossible to begin to learn that which one thinks one already knows.",
	"It is not what happens to you, but how you react to it that matters.",
	"The mind is not a vessel to be filled but a fire to be kindled.",
	"Good habits formed at youth make all the difference.",
];
static QUOTES_LEN: usize = QUOTES.len();
