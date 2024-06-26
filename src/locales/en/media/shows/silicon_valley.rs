use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn silicon_valley_characters() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_companies() -> String {
	COMPANIES[seeder::gen_range(0..COMPANIES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_quotes() -> String {
	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_apps() -> String {
	APPS[seeder::gen_range(0..APPS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_inventions() -> String {
	INVENTIONS[seeder::gen_range(0..INVENTIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_mottos() -> String {
	MOTTOS[seeder::gen_range(0..MOTTOS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_urls() -> String {
	URLS[seeder::gen_range(0..URLS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn silicon_valley_emails() -> String {
	EMAILS[seeder::gen_range(0..EMAILS_LEN)].to_string()
}

static CHARACTERS: [&'static str; 16] = [
  "Richard Hendricks",
  "Erlich Bachman",
  "Nelson Big Head Bighetti",
  "Bertram Gilfoyle",
  "Dinesh Chugtai",
  "Monica Hall",
  "Donald Jared Dunn",
  "Gavin Belson",
  "Jian Yang",
  "Laurie Bream",
  "Russ Hanneman",
  "Jack Action Jack Barker",
  "Keenan Feldspar",
  "Ed Chen",
  "Peter Gregory",
  "Ron LaFlamme"
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static COMPANIES: [&'static str; 18] = [
  "Pied Piper",
  "Hooli",
  "Raviga Capital Management",
  "Endframe",
  "Bachmanity",
  "Maleant Data Systems Solutions",
  "Aviato",
  "Coleman-Blair",
  "Raviga",
  "Yoyodyne",
  "Intersite",
  "Infotrode",
  "Bream-Hall",
  "SeeFood Technologies Inc",
  "Retinabyte",
  "VidClone Graphics",
  "Entercross Systems",
  "Turnwire"
];
static COMPANIES_LEN: usize = COMPANIES.len();

static QUOTES: [&'static str; 17] = [
  "I do not want to live in a world where someone else is making the world a better place better than we are.",
  "I firmly believe we can only achieve greatness if first, we achieve goodness",
  "Line em up, nuts to butts",
  "Let me ask you. How fast do you think you could jerk off every guy in this room? Because I know how long it would take me. And I can prove it",
  "It is weird. They always travel in groups of five. These programmers, there is always a tall, skinny white guy; short, skinny Asian guy; fat guy with a ponytail; some guy with crazy facial hair; and then an East Indian guy. It is like they trade guys until they all have the right group.",
  "Jian-Yang, what are you doing? This is Palo Alto. People are lunatics about smoking here. We do not enjoy all the freedoms that you have in China.",
  "Well, you just brought piss to a shit fight, you little cunt!",
  "Hitler actually played the bassoon. So technically Hitler was the Hitler of music.",
  "I simply imagine that my skeleton is me and my body is my house. That way I am always home.",
  "Gavin Belson started out with lofty goals too, but he just kept excusing immoral behavior just like this, until one day all that was left was a sad man with funny shoes... Disgraced, friendless, and engorged with the blood of a youthful charlatan.",
  "And that, gentlemen, is scrum. Welcome to the next eight weeks of our lives.",
  "Of course they know that you are not pitching Shazam. That already exists. This would be a Shazam... for food.",
  "Compromise is the shared hypotenuse of the conjoined triangles of success.",
  "Gentlemen, I just paid the palapa contractor. The palapa piper, so to speak. The dream is a reality. We will no longer be exposed... to the elements.",
  "I was gonna sleep last night, but, uh... I thought I had this solve for this computational trust issue I have been working on, but it turns out, I did not have a solve. But it was too late. I had already drank the whole pot of coffee.",
  "I extended my compression algorithm to support... get this... 12-bit color. Okay, so our users will be able to experience a 10 percent increase in image quality with absolutely no increase in server load whatsoever. Just-Just-Just... Just watch this. Before. After. Before. After.",
  "You listen to me, you muscle-bound handsome Adonis: tech is reserved for people like me, okay? The freaks, the weirdos, the misfits, the geeks, the dweebs, the dorks! Not you!"
];
static QUOTES_LEN: usize = QUOTES.len();

static APPS: [&'static str; 13] = [
  "Nip Alert",
  "Astraphile",
  "Panic-a-Tech",
  "Spinder",
  "Nucleus",
  "Peggd",
  "Clinkle",
  "Tables",
  "HooliChat",
  "PiperChat",
  "Not Hotdog",
  "PeaceFare",
  "CodeRag"
];
static APPS_LEN: usize = APPS.len();

static INVENTIONS: [&'static str; 17] = [
  "Telehuman",
  "Liquid Shrimp",
  "Bit Soup",
  "Audacious",
  "Tres Comas Tequila",
  "Pipey",
  "Always Blue",
  "Cold Duck",
  "Skycrane",
  "Octopus Recipes",
  "Limp Biscuit",
  "Hooli Box",
  "Box Two",
  "Table",
  "Anton",
  "BamBot",
  "Human Heater"
];
static INVENTIONS_LEN: usize = INVENTIONS.len();

static MOTTOS: [&'static str; 13] = [
  "Cloud-based, disruptive systems",
  "Creating unique cross-platform technologies",
  "Making the world a better place",
  "Awesome world-changing compression company",
  "So maybe the reason we share so much is because we understand that without sharing, we can not survive. And sharing is tables.",
  "Forced adoption through aggressive guerrilla marketing",
  "Powered by the spirit of exploration and the thrill of the pursuit of the unimaginable",
  "We not only think outside of the box, we think outside of the box that box is in - and so on - until innovation is free of all boxes that would contain and constrain it",
  "Our products are products, producing unrivaled results",
  "Oh, danger will most certainly be proceeded in the face of. Right in its face. Right in it.",
  "Isnt it time someone put the venture back into venture capital?",
  "Are bandwidth costs harshing on your vibe?",
  "The drink that does not give a fuck!"
];
static MOTTOS_LEN: usize = MOTTOS.len();

static URLS: [&'static str; 9] = [
  "http://raviga.com",
  "http://breamhall.com",
  "http://piedpiper.com",
  "http://hooli.com",
  "http://bachmanity.com",
  "http://aviato.com",
  "http://coderag.com",
  "http://endframesystems.com",
  "http://drinkhomicide.com"
];
static URLS_LEN: usize = URLS.len();

static EMAILS: [&'static str; 11] = [
  "richard@piedpiper.test",
  "bertram@piedpiper.test",
  "dinesh@piedpiper.test",
  "jared@piedpiper.test",
  "bighead@nipplealert.test",
  "erlich@bachmanity.test",
  "monica@raviga.test",
  "laurie@raviga.test",
  "gavin@hooli.test",
  "russ@threecommaclub.test",
  "denpok@hooli.test"
];
static EMAILS_LEN: usize = EMAILS.len();