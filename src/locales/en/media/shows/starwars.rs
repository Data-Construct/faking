use crate::utils::seeder;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn starwars_character() -> String {
	STARWARS_CHARACTERS[seeder::gen_range(0..STARWARS_CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_squadron() -> String {
	CALL_SQUADRONS[seeder::gen_range(0..CALL_SQUADRONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_number() -> String {
	CALL_NUMBERS[seeder::gen_range(0..CALL_NUMBERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn call_sign() -> String {
	let call_num = call_number();
	let call_squad = call_squadron();
	return concat_string!(call_num, " ", call_squad);
}

#[wasm_bindgen]
pub fn droid() -> String {
	DROIDS[seeder::gen_range(0..DROIDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn starwars_planet() -> String {
	PLANETS[seeder::gen_range(0..PLANETS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn species() -> String {
	SPECIES[seeder::gen_range(0..SPECIES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn vehicle() -> String {
	VEHICLES[seeder::gen_range(0..VEHICLES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn wookie_word() -> String {
	WOOKIE_WORDS[seeder::gen_range(0..WOOKIE_WORDS_LEN)].to_string()
}

static STARWARS_CHARACTERS: [&'static str; 60] = [
	"Padme Amidala",
	"Jar Jar Binks",
	"Borvo the Hutt",
	"Darth Caedus",
	"Boba Fett",
	"Jabba the Hutt",
	"Obi-Wan Kenobi",
	"Darth Maul",
	"Leia Organa",
	"Sheev Palpatine",
	"Kylo Ren",
	"Darth Sidious",
	"Anakin Skywalker",
	"Luke Skywalker",
	"Ben Solo",
	"Han Solo",
	"Darth Vader",
	"Watto",
	"Mace Windu",
	"Yoda",
	"Count Dooku",
	"Sebulba",
	"Qui-Gon Jinn",
	"Chewbacca",
	"Jango Fett",
	"Lando Calrissian",
	"Bail Organa",
	"Wedge Antilles",
	"Poe Dameron",
	"Ki-Adi-Mundi",
	"Nute Gunray",
	"Panaka",
	"Rune Haako",
	"Rey",
	"Finn",
	"Supreme Leader Snoke",
	"General Hux",
	"Admiral Ackbar",
	"Ahsoka Tano",
	"Asajj Ventress",
	"Bendu",
	"Captain Phasma",
	"Chirrut Imwe",
	"Ezra Bridger",
	"Galen Erso",
	"Grand Moff Tarkin",
	"Grand Admiral Thrawn",
	"Greedo",
	"Jyn Erso",
	"Lyra Erso",
	"Maz Kanata",
	"Mon Mothma",
	"Sabine Wren",
	"Saw Gerrera",
	"Savage Opress",
	"Shmi Skywalker",
	"Kanan Jarrus",
	"Hera Syndulla",
	"Rose Tico",
	"Vice Admiral Holdo",
];
static STARWARS_CHARACTERS_LEN: usize = STARWARS_CHARACTERS.len();

static CALL_SQUADRONS: [&'static str; 9] = [
	"Rogue",
	"Red",
	"Gray",
	"Green",
	"Blue",
	"Gold",
	"Black",
	"Yellow",
	"Phoenix",
];
static CALL_SQUADRONS_LEN: usize = CALL_SQUADRONS.len();

static CALL_NUMBERS: [&'static str; 6] = [
	"Leader",
	"1",
	"2",
	"3",
	"4",
	"5",
];
static CALL_NUMBERS_LEN: usize = CALL_NUMBERS.len();

static DROIDS: [&'static str; 31] = [
	"2-1B",
	"4-LOM",
	"ASP",
	"B2-RP",
	"B1",
	"BD-3000",
	"C1-10P",
	"FA-4",
	"GH-7",
	"GNK",
	"LM-432",
	"ID9",
	"11-4D",
	"2-1B",
	"327-T",
	"4-LOM",
	"B4-D4",
	"NR-N99",
	"C-3PO",
	"R2-D2",
	"BB-8",
	"R2-Q5",
	"Super Battle Droid",
	"Mouse Droid",
	"Droideka",
	"Buzz Droid",
	"Magnaguard",
	"Interrogation Droid",
	"Vulture Droid",
	"BB-9E",
	"K-2SO",
];
static DROIDS_LEN: usize = DROIDS.len();

static PLANETS: [&'static str; 35] = [
	"Alderaan",
	"Ahch-To",
	"Bespin",
	"Cantonica",
	"Coruscant",
	"Cloud City",
	"Crait",
	"DQar",
	"Dathomir",
	"Dagobah",
	"Death Star",
	"Eadu",
	"Endor",
	"Felucia",
	"Geonosis",
	"Hoth",
	"Hosnian Prime",
	"Jakku",
	"Jedha",
	"Kamino",
	"Kashyyyk",
	"Lothal",
	"Mandalore",
	"Mustafar",
	"Mygeeto",
	"Naboo",
	"Onderon",
	"Ryloth",
	"Scarif",
	"Starkiller Base",
	"Sullust",
	"Takodana",
	"Tatooine",
	"Utapau",
	"Yavin 4",
];
static PLANETS_LEN: usize = PLANETS.len();

static SPECIES: [&'static str; 15] = [
	"Ewok",
	"Hutt",
	"Gungan",
	"Ithorian",
	"Jawa",
	"Neimoidian",
	"Sullustan",
	"Wookiee",
	"Mon Calamari",
	"Bith",
	"Dathomirian",
	"Gamorreans",
	"Kaminoan",
	"Twi'lek",
	"Porg",
];
static SPECIES_LEN: usize = SPECIES.len();

static VEHICLES: [&'static str; 36] = [
	"V-Wing Fighter",
	"ATT Battle Tank",
	"Naboo N-1 Starfighter",
	"Republic Cruiser",
	"Naboo Royal Starship",
	"Gungan Bongo Submarine",
	"Flash Speeder",
	"Trade Federation Battleship",
	"Millennium Falcon",
	"Sith Infiltrator",
	"AT-ST Walker",
	"TIE Bomber",
	"Imperial Shuttle",
	"Sandcrawler",
	"TIE Interceptor",
	"Speeder Bike",
	"Death Star",
	"AT-AT Walker",
	"Imperial Star Destroyer",
	"X-Wing Fighter",
	"A-Wing Fighter",
	"GR-75 Transport",
	"Imperial Interdictor",
	"MTT",
	"Phantom II",
	"Republic Attack Gunship",
	"Rey's Speeder",
	"Ghost",
	"U-Wing",
	"Y-Wing Starfighter",
	"First Order TIE Fighter",
	"AT-M6 Walker",
	"First Order Dreadnought",
	"TIE Silencer",
	"Resistance Bomber",
	"Resistance Ski Speeder",
];
static VEHICLES_LEN: usize = VEHICLES.len();

static WOOKIE_WORDS: [&'static str; 22] = [
	"wyaaaaaa",
	"ruh",
	"huewaa",
	"muaa",
	"mumwa",
	"wua",
	"ga",
	"ma",
	"ahuma",
	"ooma",
	"youw",
	"kabukk",
	"wyogg",
	"gwyaaaag",
	"roooarrgh",
	"ur",
	"ru",
	"roo",
	"hnn-rowr",
	"yrroonn",
	"nng",
	"rarr",
];
static WOOKIE_WORDS_LEN: usize = WOOKIE_WORDS.len();

lazy_static! {
	static ref CHARACTER_QUOTES_HASHMAP: HashMap<&'static str, Vec<String>> = {
		let mut m = HashMap::new();
		m.insert(
			"admiral_ackbar",
			vec![
				"It's a trap!".to_owned(),
				"The Shield is down! Commence attack on the Death Star's main reactor.".to_owned(),
				"We have no choice, General Calrissian! Our cruisers cant repel firepower of that magnitude!".to_owned(),
			],
		);

		m.insert(
			"ahsoka_tano",
			vec![
				"Suicide is not the Jedi way, Master.".to_owned(),
				"Let's just say my master will always do what needs to be done. I'm not even sure how peacetime will agree with him.".to_owned(),
				"Sorry to interrupt your playtime, Grumpy, but wouldn't you prefer a challenge?".to_owned()
			],
		);

		m.insert(
			"anakin_skywalker",
			vec![
				"I've got a bad feeling about this.".to_owned(),
				"Just for once, let me look on you with my own eyes.".to_owned(),
				"Jedi business, go back to your drinks!".to_owned(),
			],
		);

		m.insert(
			"asajj_ventress",
			vec![
				"You're tenacious, Jedi.".to_owned(),
				"Not even the dark side can give you that power.".to_owned(),
			],
		);

		m.insert(
			"bendu",
			vec![
				"Your presence is like a violent storm in this quiet world.".to_owned(),
				"An object cannot make you good, or evil. The temptation of power, forbidden knowledge, even the desire to do good can lead some down that path. But only you can change yourself.".to_owned(),
				"Once something is known, it cannot be unknown.".to_owned()
			],
		);

		m.insert(
			"boba_fett",
			vec![
				"He's no good to me dead.".to_owned(),
				"You can run, but you'll only die tired.".to_owned(),
			],
		);

		m.insert(
			"c_3po",
			vec![
				"I have a bad feeling about this.".to_owned(),
				"R2-D2, you know better than to trust a strange computer!".to_owned(),
				"You’ll be malfunctioning within a day, you nearsighted scrap pile.".to_owned(),
				"I'm terribly sorry about all this. After all, he's only a Wookiee!".to_owned(),
				"Don’t you call me a mindless philosopher, you overweight glob of grease!".to_owned(),
				"We're doomed.".to_owned(),
				"I suggest a new strategy, R2. Let the Wookiee win.".to_owned(),
				"We seem to be made to suffer. It's our lot in life.".to_owned(),
				"I'm backwards, you filthy furball!".to_owned(),
				"If I told you half the things I've heard about this Jabba the Hutt, you'd probably short circuit.".to_owned(),
				"Don’t worry about Master Luke. I’m sure he’ll be all right. He’s quite clever, you know… for a human being.".to_owned(),
				"I can’t abide these Jawas. Disgusting creatures.".to_owned(),
				"It's against my programming to impersonate a deity.".to_owned(),
				"Just you reconsider playing that message for him! No, I don't think he likes you at all. No, I don't like you either.".to_owned(),
				"Now don't you forget this! Why I should stick my neck out for you is far beyond my capacity!".to_owned()
			],
		);

		m.insert(
			"count_dooku",
			vec![
				"Twice the pride, double the fall.".to_owned(),
				"Your swords, please. We don't want to make a mess of things in front of the Chancellor.".to_owned(),
				"What if I told you that the Republic was now under the control of a dark lord of the Sith?".to_owned()
			],
		);

		m.insert(
			"darth_caedus",
			vec![
				"You're smarter than a tree, aren't you?".to_owned(),
				"Possessions are burdens. What you have can always be taken away, so wealth breeds fear.".to_owned(),
				"I just feel—I don't know. I hated it. I wanted it out of me. I wanted it dead—but, you know, while it was in me…it made me part of something. Like in the Nursery. During the fight, it was almost like having the Force again. Now—".to_owned()
			],
		);

		m.insert(
			"darth_vader",
			vec![
				"I find your lack of faith disturbing.".to_owned(),
				"You are a member of the Rebel Alliance, and a traitor.".to_owned(),
				"You are unwise to lower your defenses!".to_owned(),
				"I am altering the deal. Pray I don't alter it any further.".to_owned(),
				"Perhaps you think you're being treated unfairly?".to_owned(),
				"The circle is now complete. When I left you, I was but the learner. Now I am the master.".to_owned(),
				"Obi-Wan was wise to hide her from me. Now, his failure is complete. If you will not turn to the Dark Side… then perhaps she will.".to_owned(),
				"Search your feelings, you know it to be true!".to_owned(),
				"Impressive. Most impressive. Obi-Wan has taught you well. You have controlled your fear. Now, release your anger. Only your hatred can destroy me.".to_owned(),
				"I hope so for your sake, Commander. The Emperor is not as forgiving as I am.".to_owned(),
				"Be careful not to choke on your aspirations, Director.".to_owned(),
				"He is as clumsy as he is stupid.".to_owned(),
				"You may use any methods necessary, but I want them alive. No disintegrations!".to_owned(),
				"You have failed me for the last time, Admiral.".to_owned()
			],
		);

		m.insert(
			"emperor_palpatine",
			vec![
				"Only now, at the end, do you understand…".to_owned(),
				"Oh, I'm afraid the deflector shield will be quite operational when your friends arrive.".to_owned(),
				"There is a great disturbance in the Force.".to_owned(),
				"Give in to your anger. With each passing moment you make yourself more my servant.".to_owned(),
				"Let the hate flow through you!".to_owned(),
				"Your feeble skills are no match for the power of the Dark Side.".to_owned(),
				"Your hate has made you powerful. Now fulfill your destiny, and take your father's place at my side.".to_owned(),
				"So be it, Jedi.".to_owned(),
				"The Force is strong with him. The son of Skywalker must not become a Jedi.".to_owned()
			],
		);

		m.insert(
			"finn",
			vec![
				"Droid, please!".to_owned(),
				"Sanitation".to_owned(),
				"Solo, we'll figure it out. We'll use the Force.".to_owned(),
				"I'm a big deal in the Resistance. Which puts a real target on my back.".to_owned(),
			],
		);

		m.insert(
			"general_hux",
			vec![
				"I won't have you question my methods.".to_owned(),
				"Careful, Ren, that your personal interests not interfere with orders from Leader Snoke.".to_owned()
			],
		);

		m.insert(
			"grand_admiral_thrawn",
			vec![
				"I will start my operations here, and pull the rebels apart piece by piece. They'll be the architects of their own destruction.".to_owned(),
				"War is in your blood. I studied the art of war, worked to perfect it, but you? You were forged by it.".to_owned()
			],
		);

		m.insert(
			"grand_moff_tarkin",
			vec![
				"Now, witness the power of this fully operational battle station.".to_owned(),
				"The Jedi are extinct. Their fire has gone out of the universe. You, my friend, are all that's left of their religion.".to_owned()
			],
		);

		m.insert(
			"greedo",
			vec![
				"Koona t'chuta Solo? (Going somewhere Solo?)".to_owned(),
				"Soong peetch alay. (It's too late.)".to_owned(),
				"Ee mara tom tee tok maky cheesa. (You should have paid him when you had the chance.)".to_owned(),
				"Jabba won neechee kochba mu shanee wy tonny wya uska. (Jabba put a price on your head so large, every bounty hunter in the galaxy will be looking for you.)".to_owned(),
				"Chosky nowy u chusu. (I'm lucky I found you first.)".to_owned(),
				"El jaya kulpa intick kuny ku suwa. (If you give it to me, I might forget I found you.)".to_owned(),
				"Semal hi teek teek. (Jabba's through with you.)".to_owned(),
				"Sone guru ye buya nyah oo won spasteega koo shu coon bon duwa weeptee. (He has no time for smugglers who drop their shipments at the first sign of an Imperial cruiser.)".to_owned(),
				"Talk Jabba. (Tell that to Jabba.)".to_owned(),
				"Boompa kom bok nee aht am bompah. (He may only take your ship.)".to_owned(),
				"Nuklee numaa (that's the idea.)".to_owned(),
				"Ches ko ba tuta creesta crenko ya kolska! (This is something I've been looking forward to for a long time.)".to_owned()
			],
		);

		m.insert(
			"han_solo",
			vec![
				"It's the ship that made the Kessel Run in less than 12 parsecs.".to_owned(),
				"She may not look like much, but she's got it where it counts, kid.".to_owned(),
				"Never tell me the odds".to_owned(),
				"Well, you said you wanted to be around when I made a mistake.".to_owned(),
				"No reward is worth this.".to_owned(),
				"Shut him up or shut him down.".to_owned(),
				"I got a bad feeling about this.".to_owned(),
				"I have a really bad feeling about this.".to_owned(),
				"Ungh. And I thought they smelled bad on the outside.".to_owned(),
				"I have a bad feeling about this.".to_owned(),
				"Bounty hunters! We don't need this scum.".to_owned(),
				"If they follow standard Imperial procedure, they'll dump their garbage before they go to light-speed.".to_owned(),
				"Hokey religions and ancient weapons are no match for a good blaster at your side, kid.".to_owned()
			],
		);

		m.insert(
			"jabba_the_hutt",
			vec![
				"Solo, la pa loiya Solo! (Solo, come out of there! Solo!)".to_owned(),
				"Bone duwa pweepway? (Have you now?)".to_owned(),
				"Han, ma bookie, keel-ee calleya ku kah. (Han, my boy, you disappoint me.)".to_owned(),
				"Wanta dah moolee-rah... (Why haven't you paid me…)".to_owned(),
				"Mon kee chees kreespa Greedo? (And why did you fry poor Greedo?)".to_owned(),
				"Han, ma kee chee zay. (Han, I cant make exceptions.)".to_owned(),
				"Hassa ba una kulkee malia... (What if everyone who smuggled for me… eveela deesa… their cargo at the sight… dwa spasteega el was nwo yana da gooloo? (of an Imperial starship?)".to_owned(),
				"Han, ma bookie, baldo nee anna dodo da eena. (You're the best.)".to_owned(),
				"See fa doi dee yaba… for an extra twenty percent… do ee deen. (Okay, fifteen percent.)".to_owned(),
				"Ee ya ba ma dookie massa... (But if you fail me again...)".to_owned(),
				"Eek bon chee ko pa na green. (I'll put a price on your head so big...)".to_owned(),
				"na meeto do buny dunko la cho ya. (you won't be able to get near a civilized system.)".to_owned(),
				"Boska! (Come on!)".to_owned()
			],
		);

		m.insert(
			"jar_jar_binks",
			vec![
				"Ooh mooey mooey I love you!".to_owned(),
				"Yoosa should follow me now, okeeday?".to_owned(),
				"Yipe! How wude!".to_owned(),
				"Ohh, maxi big da Force. Well dat smells stinkowiff.".to_owned(),
				"Oh Gooberfish!".to_owned(),
				"Exsqueeze me!".to_owned(),
				"Mesa cause one, two-y little bitty axadentes, huh? Yud say boom de gasser, den crashin der bosses heyblibber, den banished.".to_owned(),
				"Mesa called Jar-Jar Binks. Mesa your humble servant.".to_owned(),
				"My forgotten, da Bosses will do terrible tings to me, TERRRRRIBLE is me going back der!".to_owned(),
				"Mesa day startin pretty okee-day with a brisky morning munchy, then BOOM! Gettin very scared and grabbin that Jedi and POW! Mesa here! Mesa gettin' very very scared!".to_owned()
			],
		);

		m.insert(
			"k_2so",
			vec![
				"I have a bad feeling about…".to_owned(),
				"That is a bad idea. I think so, and so does Cassian. What do I know? My specialty is just strategic analysis.".to_owned(),
				"There you are. I'm standing by as you requested, although there's a problem on the horizon. There's no horizon.".to_owned()

			],
		);

		m.insert(
			"kylo_ren",
			vec![
				"You need a teacher. I can show you the ways of the Force.".to_owned(),
				"Show me again, Grandfather, and I will finish what you started.".to_owned(),
			],
		);

		m.insert(
			"lando_calrissian",
			vec![
				"Why you slimy, double-crossing, no-good swindler. You've got a lot of guts coming here, after what you pulled.".to_owned(),
				"How you doin' Chewbacca? Still hanging around with this loser?".to_owned(),
				"But how could they be jamming us if they don't know that we're coming?".to_owned(),
				"This deal is getting worse all the time.".to_owned()
			],
		);

		m.insert(
			"leia_organa",
			vec![
				"You do have your moments. Not many, but you have them.".to_owned(),
				"I have a bad feeling about this.".to_owned(),
				"Would somebody get this big walking carpet out of my way?".to_owned(),
				"Aren't you a little short for a Stormtrooper?".to_owned(),
				"Help me Obi-Wan Kenobi. You're my only hope.".to_owned(),
				"Why, you stuck-up, half-witted, scruffy-looking nerf herder!".to_owned(),
				"Governor Tarkin, I should've expected to find you holding Vader's leash. I recognized your foul stench when I was brought on board.".to_owned(),
				"Somebody has to save our skins. Into the garbage chute, flyboy!".to_owned()
			],
		);

		m.insert(
			"luke_skywalker",
			vec![
				"But I was going into Tosche Station to pick up some power converters!".to_owned(),
				"I have a very bad feeling about this.".to_owned(),
				"It's not impossible. I used to bullseye womp rats in my T-16 back home; they're not much bigger than two meters.".to_owned(),
				"You know, that little droid is going to cause me a lot of trouble.".to_owned(),
				"If you're saying that coming here was a bad idea, I'm starting to agree with you.".to_owned(),
				"You'll find I'm full of surprises!".to_owned(),
				"Your overconfidence is your weakness.".to_owned(),
				"You serve your master well. And you will be rewarded.".to_owned(),
				"Threepio, tell them if they don't do as you wish, you'll become angry and use your magic.".to_owned(),
				"I am a Jedi, like my father before me.".to_owned()
			],
		);

		m.insert(
			"mace_windu",
			vec![
				"The senate will decide your fate.".to_owned(),
				"Then our worst fears have been realized. We must move quickly if the Jedi Order is to survive.".to_owned()
			],
		);

		m.insert(
			"maz_kanata",
			vec!["I assume you need something. Desperately.".to_owned()],
		);

		m.insert(
			"obi_wan_kenobi",
			vec![
				"An elegant weapon for a more civilized age.".to_owned(),
				"You don’t need to see his identification. These aren’t the droids you’re looking for.".to_owned(),
				"You will never find a more wretched hive of scum and villainy. We must be cautious.".to_owned(),
				"Who's the more foolish; the fool, or the fool who follows him?".to_owned(),
				"I have a bad feeling about this.".to_owned(),
				"Strike me down, and I will become more powerful than you could possibly imagine.".to_owned(),
				"In my experience there is no such thing as luck.".to_owned(),
				"The Force will be with you. Always.".to_owned(),
				"That's no moon. It's a space station.".to_owned(),
				"I felt a great disturbance in the Force, as if millions of voices suddenly cried out in terror and were suddenly silenced.".to_owned(),
				"Use the Force, Luke.".to_owned()
			],
		);

		m.insert(
			"padme_amidala",
			vec![
				"So this is how liberty dies. With thunderous applause.".to_owned(),
				"Ani? My goodness, you've grown.".to_owned(),
				"Anakin, you're breaking my heart. You're going down a path I cant follow.".to_owned(),
				"Hold me, like you did by the lake on Naboo; so long ago when there was nothing but our love. No politics, no plotting, no war.".to_owned(),
				"I was not elected to watch my people suffer and die while you discuss this invasion in a committee!".to_owned()
			],
		);

		m.insert(
			"qui_gon_jinn",
			vec!["Remember, your focus determines your reality.".to_owned()],
		);

		m.insert(
			"rey",
			vec![
				"You will remove these restraints and leave this cell with the door open."
					.to_owned(),
				"The garbage'll do".to_owned(),
			],
		);

		m.insert(
			"shmi_skywalker",
			vec![
				"You cant stop change any more than you can stop the suns from setting."
					.to_owned(),
				"The Republic doesn't exist out here. We must survive on our own.".to_owned(),
			],
		);

		m.insert(
			"yoda",
			vec![
				"Wars not make one great.".to_owned(),
				"Truly wonderful, the mind of a child is.".to_owned(),
				"That is why you fail.".to_owned(),
				"A Jedi uses the Force for knowledge and defense, never for attack.".to_owned(),
				"Adventure. Excitement. A Jedi craves not these things.".to_owned(),
				"Fear is the path to the dark side. Fear leads to anger… anger leads to hate… hate leads to suffering.".to_owned(),
				"Judge me by my size, do you?".to_owned(),
				"Do. Or do not. There is no try.".to_owned(),
				"Luminous beings are we… not this crude matter.".to_owned(),
				"Train yourself to let go of everything you fear to lose.".to_owned(),
				"Size matters not. Look at me. Judge me by my size, do you?".to_owned(),
				"Ohhh. Great warrior. Wars not make one great.".to_owned()
			],
		);
		m
	};
}

#[wasm_bindgen]
pub fn admiral_ackbar_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"admiral_ackbar").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn ahsoka_tano_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"ahsoka_tano").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn anakin_skywalker_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"anakin_skywalker").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn asajj_ventress_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"asajj_ventress").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn bendu_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"bendu").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn boba_fett_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"boba_fett").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn c_3po_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"c_3po").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn count_dooku_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"count_dooku").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn darth_caedus_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"darth_caedus").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn darth_vader_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"darth_vader").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn emperor_palpatine_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"emperor_palpatine").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn finn_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"finn").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn general_hux_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"general_hux").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn grand_admiral_thrawn_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP
		.get(&"grand_admiral_thrawn")
		.unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn grand_moff_tarkin_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"grand_moff_tarkin").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn greedo_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"greedo").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn han_solo_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"han_solo").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn jabba_the_hutt_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"jabba_the_hutt").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn jar_jar_binks_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"jar_jar_binks").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn k_2so_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"k_2so").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn kylo_ren_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"kylo_ren").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn lando_calrissian_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"lando_calrissian").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn leia_organa_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"leia_organa").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn luke_skywalker_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"luke_skywalker").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn mace_windu_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"mace_windu").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn maz_kanata_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"maz_kanata").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn obi_wan_kenobi_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"obi_wan_kenobi").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn padme_amidala_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"padme_amidala").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn qui_gon_jinn_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"qui_gon_jinn").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn rey_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"rey").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn shmi_skywalker_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"shmi_skywalker").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}

#[wasm_bindgen]
pub fn yoda_quote() -> String {
	let aa = CHARACTER_QUOTES_HASHMAP.get(&"yoda").unwrap();
	aa[seeder::gen_range(0..aa.len())].to_string()
}
