use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn spongebob_character() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn spongebob_location() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn spongebob_quote() -> String {
	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn spongebob_episode_title() -> String {
	EPISODE_TITLES[seeder::gen_range(0..EPISODE_TITLES_LEN)].to_string()
}
static CHARACTERS: [&'static str; 68] = [
	"SpongeBob SquarePants",
	"Patrick Star",
	"Squidward Tentacles",
	"Mr. Krabs",
	"Plankton and Karen",
	"Sandy Cheeks",
	"Mrs. Puff",
	"Pearl Krabs",
	"Patchy the Pirate",
	"Potty the Parrot",
	"French Narrator",
	"Barnacle Boy",
	"Mermaid Man",
	"King Neptune",
	"Larry the Lobster",
	"Harold and Margaret SquarePants",
	"Realistic Fish Head",
	"Perch Perkins",
	"Bubble Bass",
	"Fred",
	"Don the Whale",
	"Harold",
	"Nat",
	"Officer Nancy",
	"Old Man Jenkins",
	"Dr. Gill Gilliam",
	"Jellyfish",
	"Angry Jack",
	"Beatrice",
	"Bob Barnacle",
	"Bubble Buddy",
	"Craig Mammalton",
	"Cuddle E. Hugs",
	"Dirty Bubble",
	"DoodleBob",
	"Flats the Flounder",
	"Gene Scallop",
	"Gordon",
	"Granny",
	"Jack Kahuna Laguna",
	"Jim",
	"Kenny the Cat",
	"Lord Royal Highness",
	"Man Ray",
	"Master Udon",
	"The Moth",
	"Nicholas Withers",
	"Queen Amphitrite",
	"Prince Triton",
	"Sal",
	"Santa Claus",
	"The Sea Bear",
	"Sea Monster",
	"Sergeant Sam Roderick",
	"Sharkface",
	"Spot",
	"Squilliam Fancyson III",
	"The Tattletale Strangler",
	"The Warden",
	"Princess Mindy",
	"Dennis",
	"The Cyclops",
	"Burger-Beard the Pirate",
	"Kyle",
	"Bubbles",
	"King Poseidon",
	"Sage",
	"El Diablo",
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static LOCATIONS: [&'static str; 28] = [
	"Bikini Bottom",
	"Rock Bottom",
	"Bottoms Up",
	"Tentacle Acres",
	"New Kelp City",
	"Bass Vegas",
	"Dullsville",
	"Far-Out-Ville",
	"Atlantis",
	"Shell City",
	"Encino, California",
	"Bubbletown",
	"Ukulele Bottom",
	"Leisure Village",
	"Palm Bay",
	"Barnacle Bay",
	"Bikinos Bottos",
	"Mu",
	"Squirrel City",
	"El Tuna",
	"Base Eight",
	"Stupidtown ",
	"Quittersville",
	"Failuretown",
	"Loserburg",
	"Farawayville",
	"We Love Musictown",
	"Crush County",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 123] = [
	"Home is where you are surrounded by other critters that care about you.",
	"If you believe in yourself and with a tiny pinch of magic, all your dreams can come true.",
	"Sometimes we have to go deep inside ourselves to solve our problems.",
	"I will have you know that I stubbed by toe last week and only cried for 20 minutes.",
	"It is not always what you say that matters, sometimes it is what you do not say.",
	"I know of a place where you never get harmed. A magical place with magical charm. Indoors. Indoors. Indoors!",
	"Good people do not rip other peoples arms off.",
	"Dumb people are always blissfully unaware of how dumb they really are…",
	"You are a man now, SpongeBob, and it is time you started acting like one.",
	"I am ugly and I am proud.",
	"That is it mister! You just lost your brain privileges!",
	"Excuse me, sir, but you are sitting on my body, which is also my face.",
	"This is not your average, everyday darkness. This is…ADVANCED darkness. Hey, if I close my eyes it does not seem so dark.",
	"Too bad SpongeBobs not here to enjoy Spongebob not being here.",
	"With imagination, you can be anything you want.",
	"Well, it is no secret that the best thing about a secret is secretly telling someone your secret, thereby adding another secret to their secret collection of secrets, secretly.",
	"You will never get what you want if you always let people step on you.",
	"Run Mr. Krabs! Run like you are not in a coma!",
	"I guess hibernation is the opposite of beauty sleep!",
	"Moss always points to civilization.",
	"The inner machinations of my mind are an enigma.",
	"SpongeBob is the only guy I know who can have fun with a jellyfish…for 12 hours!",
	"Do not you DARE take the name of Texas in vain.",
	"Always follow your heart,",
	"See, no one says cool anymore. That is such an old person thing. Now we say coral, as in That nose job is so coral.",
	"Well, it may be stupid, but it is also dumb.",
	"You can not fool me. I listen to public radio!",
	"If there is one thing we Atlanteans enjoy, it is a healthy dose of dark humor!",
	"You do not need a license to drive a sandwich",
	"Holographic Meatloaf? My favorite!",
	"Oh well, I guess I am not wearing any pants today!",
	"We shall never deny a guest, even the most ridiculous request.",
	"I am so loyal, I do not mind sleeping out in the cold, hard ground while Captain Krabs sleeps in his warm, dry tent.",
	"A five letter word for happiness…money!",
	"You never really know the true value of a moment, until it becomes a memory.",
	"Well, the way I see it, there are three possibilities: One, you stole it; two, you stole it; or three, you stole it!",
	"If I were to die right now in a fiery explosion due to the carelessness of a friend…Then it would just be alright.",
	"Knowledge cannot replace friendship.",
	"Can I be excused for the rest of my life?",
	"Nonsense, my vocabulary is infinitely expanding!",
	"The best time to wear a striped sweater…is all the time!",
	"Too bad that did not kill me.",
	"I am a good noodle!",
	"We do not need television…not as long as we have our imagination.",
	"Being grown up is boring. Besides, I do not get jazz.",
	"F is for friends who do stuff together!",
	"Is mayonnaise an instrument?",
	"Once upon a time there was an ugly barnacle. He was so ugly that everyone died. The end!",
	"I wumbo, you wumbo, he she we wumbo.",
	"Wake me up when I care.",
	"It is just a cruel reminder that I am single and likely to remain that way forever.",
	"Did you smell it? That smell. A kind of smelly smell. The smelly smell that smells…smelly.",
	"I knew I should not have gotten out of bed today.",
	"Goodbye everyone, I will remember you all in therapy.",
	"I can not see my forehead!",
	"But it is my only night to be fancy!",
	"All I know is fine dining and breathing.",
	"Look at all the hip young people eating sal-ads.",
	"We should take Bikini Bottom and push it somewhere else!",
	"Remember, licking doorknobs is illegal on other planets.",
	"Patrick, do not you have to be stupid somewhere else?",
	"Who are you people?!",
	"Squidward that is not the peace treaty, that is a copy of the peace treaty.",
	"After going on your life-changing journey, you now realize that you do not want what you thought you wanted. What you really wanted was inside you all along.",
	"Karen, baby, I have not felt this giddy since the day you agreed to be my wife!",
	"That is a big boot.",
	"Oh Karen, my computer wife, if only I could have managed to steal the secret to Krabs success. The formula for the Krabby Patty…Ohhh…Then people would line up to eat at MY restaurant! Lord knows I have tried. I have exhausted every evil plan in my filing cabinet…from A to Y!",
	"Patrick, you have been wearing the same Goofy Goober Peanut Party underpants for three years straight! What do you call that?",
	"It is evil. It is diabolical. It is lemon-scented. This Plan Z can not possibly fail!",
	"Pull your pants up, Patrick. We are going home.",
	"I have worked for Mr. Krabs for many years and always thought he was a great boss.",
	"A Triple Gooberberry Sunrise, huh? I guess I could use one of those.",
	"It started out as a simple order. A Krabby Patty with cheese. When the customer took a bite, NO CHEESE!",
	"And tonight, after my big promotion, we are gonna party till we are purple.",
	"One hundred and one dollars for a Krabby Patty?",
	"We paid nine dollars for this?",
	"My pants are on fire!…My underwear is on fire!…IM ON FIRE!",
	"His chops are too righteous. The helmets can not handle this level of rock n roll! Karen, do something!",
	"Hello? Whered everybody go? Did I miss something? Did you see my butt?",
	"That is not my wallet!",
	"Does this look unsure to you?",
	"It is a giraffe!",
	"Where is the leak maam?",
	"The Krusty Krab pizza, is a pizza, for you and me!",
	"Firmly grasp it in your hand",
	"E minor! Yeah, alright!",
	"You forgot the pickle!",
	"I think do not ready go back to work Mr. Krabs.",
	"Listen here you crustacous cheapskate!",
	"I am taking jelly off the menu.",
	"Spongebob! The aliens would like to have a word with you!",
	"F is for fire which burns down the city, U is for uranium... Bombs! N is for no survivors!",
	"Mr. Krabs why did you eat my boots?",
	"Do not touch me, I am sterile!",
	"I defy you, heartman!",
	"Heart on stick must die!",
	"Come on Mr. Krabs, show a little dignity.",
	"Can we say that shoes from Texas are dumb?",
	"Get that uniform on, sailor! It is already been paid for!",
	"At least I am safe inside my mind.",
	"No, this is Patrick!",
	"Hey pal, you just fly in from stupidtown?",
	"Remember: Ravioli, Ravioli, give me the formioli!",
	"I did not bring christmas to bikini bottom spongebob, you did!",
	"Who you callin Pinhead?",
	"We stole a balloon!",
	"Not even... Squidwards House!",
	"We understand you have a dying animal...",
	"Whoevers the owner of the white sedan, you left your lights on.",
	"Maybe we should play so quietly nobody can hear us.",
	"Big, meaty claws!",
	"No people, let us be smart and bring it off.",
	"Just look at his daughter, she is as big as a whale!",
	"What? It is just an ordinary Krabby Patt - Oh my goodness! Squidward!",
	"You like Krabby Patties, do not you squidward?",
	"I have no soul!",
	"Next you will be asking me to go square dancing with Patrick!",
	"Because you and me - We are like brothers. Only closer.",
	"So much time later the narrator got tired so they hired a new narrator",
	"Two things that will not work!",
	"Oh no he is hot!",
	"We are not cavemen! We have technology!",
	"Actually, it is coral blue number 3...",
];
static QUOTES_LEN: usize = QUOTES.len();

static EPISODE_TITLES: [&'static str; 557] = [
	"Help Wanted",
	"Reef Blower",
	"1999Plankton!",
	"Pizza Delivery",
	"Home Sweet Pineapple",
	"Mermaid Man and Barnacle Boy",
	"Pickles",
	"Jellyfish Jam",
	"Sandys Rocket",
	"Nature Pants",
	"Opposite Day",
	"Culture Shock",
	"F.U.N",
	"MuscleBob BuffPants",
	"Squidward the Unfriendly Ghost",
	"The Chaperone",
	"Employee of the Month",
	"Scaredy Pants",
	"I Was a Teenage Gary",
	"SB-129",
	"Karate Choppers",
	"Sleepy Time",
	"Suds",
	"Valentines Day",
	"The Paper",
	"Arrgh",
	"Rock Bottom",
	"Texas",
	"Walking Small",
	"Fools in April",
	"Neptunes Spatula",
	"Hooky",
	"Mermaid Man and Barnacle Boy II",
	"Your Shoes Untied",
	"Squids Day Off",
	"Something Smells",
	"Bossy Boots",
	"Big Pink Loser",
	"Bubble Buddy",
	"Dying for Pie",
	"Imitation Krabs",
	"Wormy",
	"Patty Hype",
	"Grandmas Kisses",
	"Squidville",
	"Prehibernation Week",
	"Life of Crime",
	"Patchy the Pirate Commercial",
	"Survival of the Idiots",
	"Dumped",
	"No Free Rides",
	"I am Your Biggest Fanatic",
	"Mermaid Man and Barnacle Boy III",
	"Squirrel Jokes",
	"Pressure",
	"The Smoking Peanut",
	"You Wish",
	"Gary Takes a Bath",
	"Welcome to the Chum Bucket",
	"Frankendoodle",
	"The Secret Box",
	"Band Geeks",
	"Graveyard Shift",
	"Krusty Love",
	"Procrastination",
	"I am with Stupid",
	"Sailor Mouth",
	"Artist Unknown",
	"Jellyfish Hunter",
	"The Fry Cook Games",
	"Squid on Strike",
	"Sandy, SpongeBob, and the Worm",
	"The Algaes Always Greener",
	"SpongeGuard on Duty",
	"Club SpongeBob",
	"My Pretty Seahorse",
	"Just One Bite",
	"The Bully",
	"Nasty Patty",
	"Idiot Box",
	"Mermaid Man and Barnacle Boy IV",
	"Doing Time",
	"Snowball Effect",
	"One Krabs Trash",
	"As Seen on TV",
	"Can You Spare a Dime",
	"No Weenies Allowed",
	"Squilliam Returns",
	"Krab Borg",
	"Rock-a-Bye Bivalve",
	"Wet Painters",
	"Krusty Krab Training Video",
	"Patchy for House Party",
	"Chocolate with Nuts",
	"Mermaid Man and Barnacle Boy V",
	"New Student Starfish",
	"Clams",
	"Patchy for B.",
	"The Great Snail Race",
	"Mid-Life Crustacean",
	"Born Again Krabs",
	"I Had an Accident",
	"Krabby Land",
	"The Camping Episode",
	"Missing Identity",
	"Planktons Army",
	"Patchys Lost Episode",
	"SpongeBob Meets the Strangler",
	"Pranks a Lot",
	"Fear of a Krabby Patty",
	"Shell of a Man",
	"The Lost Mattress",
	"Krabs vs. Plankton",
	"Have You Seen This Snail",
	"Skill Crane",
	"Good Neighbors",
	"Selling Out",
	"Funny Pants",
	"Dunces and Dragons",
	"Enemy In-Law",
	"Mermaid Man & Barnacle Boy VI: The Motion Picture",
	"Patrick SmartPants",
	"SquidBob TentaclePants",
	"Krusty Towers",
	"Mrs. Puff, You are Fired",
	"Chimps Ahoy",
	"Ghost Host",
	"Whale of a Birthday",
	"Karate Island",
	"All That Glitters",
	"Wishing You Well",
	"New Leaf",
	"Once Bitten",
	"Bummer Vacation",
	"Wigstruck",
	"Squidtastic Voyage",
	"That is No Lady",
	"The Thing",
	"Hocus Pocus",
	"Driven to Tears",
	"Rule of Dumb",
	"Born to Be Wild",
	"Best Frenemies",
	"The Pink Purloiner",
	"Squid Wood",
	"Best Day Ever",
	"The Gift of Gum",
	"Friend or Foe",
	"The Original Fry Cook",
	"Night Light",
	"Rise and Shine",
	"Waiting",
	"Fungus Among Us",
	"Spy Buddies",
	"Boat Smarts",
	"Good Ol Whatshisname",
	"New Digs",
	"Krabs à la Mode",
	"Roller Cowards",
	"Bucket Sweet Bucket",
	"To Love a Patty",
	"Breath of Fresh Squidward",
	"Money Talks",
	"SpongeBob vs. The Patty Gadget",
	"Slimy Dancing",
	"The Krusty Sponge",
	"Sing a Song of Patrick",
	"A Flea in Her Dome",
	"The Donut of Shame",
	"The Krusty Plate",
	"Goo Goo Gas",
	"Le Big Switch",
	"Atlantis SquarePantis",
	"Picture Day",
	"Pat No Pay",
	"BlackJack",
	"Blackened Sponge",
	"Mermaid Man vs. SpongeBob",
	"The Inmates of Summer",
	"To Save a Squirrel",
	"Pest of the West",
	"20,000 Patties Under the Sea",
	"The Battle of Bikini Bottom",
	"What Ever Happened to SpongeBob",
	"The Two Faces of Squidward",
	"SpongeHenge",
	"Banned in Bikini Bottom",
	"Stanley S. SquarePants",
	"House Fancy",
	"Krabby Road",
	"Penny Foolish",
	"Nautical Novice",
	"Spongicus",
	"Suction Cup Symphony",
	"Not Normal",
	"Gone",
	"The Splinter",
	"Slide Whistle Stooges",
	"A Life in a Day",
	"Sun Bleached",
	"Giant Squidward",
	"No Nose Knows",
	"Patty Caper",
	"Planktons Regular",
	"Boating Buddies",
	"The Krabby Kronicle",
	"The Slumber Party",
	"Grooming Gary",
	"SpongeBob SquarePants vs. The Big One",
	"Porous Pockets",
	"Choir Boys",
	"Krusty Krushers",
	"The Card",
	"Dear Vikings",
	"Ditchin",
	"Grandpappy the Pirate",
	"Cephalopod Lodge",
	"Squids Visit",
	"To SquarePants or Not to SquarePants",
	"Shuffleboarding",
	"Professor Squidward",
	"Pet or Pests",
	"Komputer Overload",
	"Gullible Pants",
	"Overbooked",
	"No Hat for Pat",
	"Toy Store of Doom",
	"Sand Castles in the Sand",
	"Shell Shocked",
	"Chum Bucket Supreme",
	"Single Cell Anniversary",
	"Truth or Square",
	"Pineapple Fever",
	"Chum Caverns",
	"The Clash of Triton",
	"Tentacle-Vision",
	"I <3 Dancing",
	"Growth Spout",
	"Stuck in the Wringer",
	"Someones in the Kitchen with Sandy",
	"The Inside Job",
	"Greasy Buffoons",
	"Model Sponge",
	"Keep Bikini Bottom Beautiful",
	"A Pal for Gary",
	"Yours, Mine and Mine",
	"Kracked Krabs",
	"The Curse of Bikini Bottom",
	"Squidward in Clarinetland",
	"SpongeBobs Last Stand",
	"Back to the Past",
	"The Bad Guy Club for Villains",
	"A Day Without Tears",
	"Summer Job",
	"One Coarse Meal",
	"Gary in Love",
	"The Plays the Thing",
	"Rodeo Daze",
	"Grammas Secret Recipe",
	"The Cent of Money",
	"The Monster Who Came to Bikini Bottom",
	"Welcome to the Bikini Bottom Triangle",
	"The Curse of the Hex",
	"The Main Drain",
	"Trenchbillies",
	"Sponge-Cano",
	"The Great Patty Caper",
	"That Sinking Feeling",
	"Karate Star",
	"Buried in Time",
	"Enchanted Tiki Dreams",
	"The Abrasive Side",
	"Earworm",
	"Hide and Then What Happens",
	"Shellback Shenanigans",
	"The Masterpiece",
	"Whelk Attack",
	"You Do not Know Sponge",
	"Tunnel of Glove",
	"Krusty Dogs",
	"The Wreck of the Mauna Loa",
	"New Fish in Town",
	"Love That Squid",
	"Big Sister Sam",
	"Perfect Chemistry",
	"Accidents Will Happen",
	"The Other Patty",
	"Drive Thru",
	"The Hot Shot",
	"A Friendly Game",
	"Sentimental Sponge",
	"Frozen Face-Off",
	"Squidwards School for Grown-Ups",
	"Oral Report",
	"Sweet and Sour Squid",
	"The Googly Artiste",
	"A SquarePants Family Vacation",
	"Patricks Staycation",
	"Walking the Plankton",
	"Mooncation",
	"Mr. Krabs Takes a Vacation",
	"Ghoul Fools",
	"Mermaid Man Begins",
	"Planktons Good Eye",
	"Barnacle Face",
	"Pet Sitter Pat",
	"House Sittin for Sandy",
	"Smoothe Jazz at Bikini Bottom",
	"Bubble Troubles",
	"The Way of the Sponge",
	"The Krabby Patty That Ate Bikini Bottom",
	"Bubble Buddy Returns",
	"Restraining SpongeBob",
	"Fiasco",
	"Are You Happy Now",
	"Planet of the Jellyfish",
	"Free Samples",
	"Home Sweet Rubble",
	"Karen 2.0",
	"InSPONGEiac",
	"Face Freeze",
	"Glove World R.I.P",
	"Squiditis",
	"Demolition Doofus",
	"Treats",
	"For Here or to Go",
	"It is a SpongeBob Christmas",
	"Super Evil Aquatic Villain Team Up is Go",
	"Chum Fricassee",
	"The Good Krabby Name",
	"Move It or Lose It",
	"Hello Bikini Bottom",
	"Extreme Spots",
	"Squirrel Record",
	"Patrick-Man",
	"Garys New Toy",
	"License to Milkshake",
	"Squid Baby",
	"Little Yellow Book",
	"Bumper to Bumper",
	"Eek, an Urchin",
	"Squid Defense",
	"Jailbreak",
	"Evil Spatula",
	"It Came from Goo Lagoon",
	"Safe Deposit Krabs",
	"Planktons Pet",
	"Do not Look Now",
	"Séance Shméance",
	"Kenny the Cat",
	"Yeti Krabs",
	"SpongeBob You are Fired",
	"Lost in Bikini Bottom",
	"Tutor Sauce",
	"Squid Plus One",
	"The Executive Treatment",
	"Company Picnic",
	"Pull Up a Barrel",
	"Sanctuary",
	"What is Eating Patrick",
	"Patrick! The Game",
	"The Sewers of Bikini Bottom",
	"SpongeBob LongPants",
	"Larrys Gym",
	"The Fish Bowl",
	"Married to Money",
	"Mall Girl Pearl",
	"Two Thumbs Down",
	"Sharks vs. Pods",
	"CopyBob DittoPants",
	"Sold",
	"Lame and Fortune",
	"Goodbye, Krabby Patty",
	"Sandys Nutmare",
	"Bulletin Board",
	"Food Con Castaways",
	"Snail Mail",
	"Pineapple Invasion",
	"Salsa Imbecilicus",
	"Mutiny on the Krusty",
	"The Whole Tooth",
	"Whirly Brains",
	"Mermaid Pants",
	"Unreal Estate",
	"Code Yellow",
	"Mimic Madness",
	"House Worming",
	"Snooze You Lose",
	"Krusty Katering",
	"SpongeBobs Place",
	"Plankton Gets the Boot",
	"Life Insurance",
	"Burst Your Bubble",
	"Plankton Retires",
	"Trident Trouble",
	"The Incredible Shrinking Sponge",
	"Sportz",
	"The Getaway",
	"Lost and Found",
	"Patricks Coupon",
	"Out of the Picture",
	"Feral Friends",
	"Do not Wake Patrick",
	"Cave Dwelling Sponge",
	"The Clam Whisperer",
	"Spot Returns",
	"The Check-Up",
	"Spin the Bottle",
	"There is a Sponge in My Soup",
	"Man Ray Returns",
	"Larry the Floor Manager",
	"The Legend of Boo-Kini Bottom",
	"No Pictures Please",
	"Stuck on the Roof",
	"Krabby Patty Creature Feature",
	"Teachers Pests",
	"Sanitation Insanity",
	"Bunny Hunt",
	"Squid Noir",
	"Scavenger Pants",
	"Cuddle E. Hugs",
	"Pat the Horse",
	"Chatterbox Gary",
	"Do not Feed the Clowns",
	"Drive Happy",
	"Old Man Patrick",
	"Fun-Sized Friends",
	"Grandmums the Word",
	"Doodle Dimension",
	"Moving Bubble Bass",
	"High Sea Diving",
	"Bottle Burglars",
	"My Leg",
	"Ink Lemonade",
	"Mustard O Mine",
	"Shopping List",
	"Whale Watching",
	"Krusty Kleaners",
	"Patnocchio",
	"ChefBob",
	"Plankton Paranoia",
	"Library Cards",
	"Call the Cops",
	"Surf N Turf",
	"Goons on the Moon",
	"Appointment TV",
	"Karens Virus",
	"The Grill is Gone",
	"The Night Patty",
	"Bubbletown",
	"Girls Night Out",
	"Squirrel Jelly",
	"The String",
	"FarmerBob",
	"Gary & Spot",
	"The Nitwitting",
	"The Ballad of Filthy Muck",
	"The Krusty Slammer",
	"Pineapple RV",
	"Garys Got Legs",
	"King Plankton",
	"Planktons Old Chum",
	"Stormy Weather",
	"Swamp Mates",
	"One Trick Sponge",
	"The Krusty Bucket",
	"Squids on a Bus",
	"Sandys Nutty Nieces",
	"Insecurity Guards",
	"Broken Alarm",
	"Karens Baby",
	"Shell Games",
	"Senior Discount",
	"Mind the Gap",
	"Dirty Bubble Returns",
	"Jolly Lodgers",
	"Biddy Sitting",
	"SpongeBobs Big Birthday Blowout",
	"SpongeBob in RandomLand",
	"SpongeBobs Bad Habit",
	"Handemonium",
	"Breakin",
	"Boss for a Day",
	"The Goofy Newbie",
	"The Ghost of Plankton",
	"My Two Krabses",
	"Knock Knock, Whos There",
	"Pat Hearts Squid",
	"Lighthouse Louie",
	"Hiccup Plague",
	"A Cabin in the Kelp",
	"The Hankering",
	"Who R Zoo",
	"Kwarantined Krab",
	"Planktons Intern",
	"Patricks Tantrum",
	"Bubble Basss Tab",
	"Kooky Cooks",
	"Escape from Beneath Glove World",
	"Krusty Koncessionaires",
	"Dream Hoppers",
	"A Place for Pets",
	"Lockdown for Love",
	"Under the Small Top",
	"Squidwards Sick Daze",
	"Goofy Scoopers",
	"Pat the Dog",
	"Something Narwhal This Way Comes",
	"C.H.U.M.S",
	"SpongeBobs Road to Christmas",
	"Potato Puff",
	"There Will Be Grease",
	"The Big Bad Bubble Bass",
	"Sea-Man Sponge Haters Club",
	"Food PBFFT! Truck",
	"Upturn Girls",
	"Say Awww",
	"Patrick the Mailman",
	"Captain Pipsqueak",
	"Plane to Sea",
	"Squidferatu",
	"Slappy Daze",
	"Welcome to Binary Bottom",
	"You are Going to Pay...Phone",
	"Abandon Twits",
	"Wallhalla",
	"Salty Sponge",
	"Karen for Spot",
	"Arbor Day Disarray",
	"Aint That the Tooth",
	"Ma and Pas Big Hurrah",
	"Yellow Pavement",
	"The Flower Plot",
	"SpongeBob on Parade",
	"Delivery to Monster Island",
	"Ride Patrick Ride",
	"Hot Crossed Nuts",
	"Sir Urchin and Snail Fail",
	"Friendiversary",
	"Mandatory Music",
	"Dopey Dick",
	"Plankton and the Beanstalk",
	"My Friend Patty",
	"FUN-Believable",
	"Spatula of the Heavens",
	"Garys Playhouse",
	"Swimming Fools",
	"The Goobfather",
	"SquidBird",
	"Allergy Attack",
	"Big Top Flop",
	"Sandy, Help Us",
	"Single-Celled Defense",
	"Buff for Puff",
	"We <3 Hoops",
	"SpongeChovy",
	"BassWard",
];
static EPISODE_TITLES_LEN: usize = EPISODE_TITLES.len();
