use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simpsons_character() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn simpsons_location() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn simpsons_quote() -> String {
	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn simpsons_episode_title() -> String {
	EPISODE_TITLES[seeder::gen_range(0..EPISODE_TITLES_LEN)].to_string()
}
static CHARACTERS: [&'static str; 165] = [
	"Homer Simpson",
	"Marge Simpson",
	"Bart Simpson",
	"Lisa Simpson",
	"Maggie Simpson",
	"Akira",
	"Ms. Albright",
	"Aristotle Amadopolis",
	"Atkins, State Comptroller",
	"Mary Bailey",
	"Birchibald Birch T. Barlow",
	"Jasper Beardly",
	"Benjamin",
	"Doug",
	"Gary",
	"Bill",
	"Marty",
	"Blinky",
	"Blue Haired Lawyer",
	"Boobarella",
	"Wendell Borton",
	"Jacqueline Bouvier",
	"Ling Bouvier",
	"Patty Bouvier",
	"Selma Bouvier",
	"Kent Brockman",
	"Bumblebee Man",
	"Charles Montgomery Burns",
	"Capital City Goofball",
	"Carl Carlson",
	"Cesar",
	"Ugolin",
	"Crazy Cat Lady",
	"Superintendent Gary Chalmers",
	"Shauna Chalmers",
	"Charlie",
	"Chase",
	"Scott Christian",
	"Comic Book Guy",
	"Mr. Costington",
	"Database",
	"Declan Desmond",
	"Disco Stu",
	"Dolph",
	"Lunchlady Doris",
	"Duffman",
	"Eddie",
	"Lou",
	"Ernst",
	"Gunter",
	"Fat Tony",
	"Maude Flanders",
	"Ned Flanders",
	"Rod Flanders",
	"Todd Flanders",
	"Francesca",
	"Frankie the Squealer",
	"Professor John Frink",
	"Baby Gerald",
	"Ginger Flanders",
	"Gino",
	"Mrs. Glick",
	"Gloria",
	"Barney Gumble",
	"Gil Gunderson",
	"Judge Constance Harm",
	"Herman Hermann",
	"Bernice Hibbert",
	"Dr. Julius Hibbert",
	"Elizabeth Hoover",
	"Lionel Hutz",
	"Itchy",
	"Scratchy",
	"Jacques",
	"Jimbo Jones",
	"Joey",
	"Rachel Jordan",
	"Kang",
	"Kodos",
	"Princess Kashmir",
	"Kearney Zzyzwicz",
	"Kearney Zzyzwicz Jr.",
	"Edna Krabappel",
	"Rabbi Hyman Krustofski",
	"Krusty the Clown",
	"Cookie Kwan",
	"Dewey Largo",
	"Legs",
	"Louie",
	"Leopold",
	"Lenny Leonard",
	"Lewis",
	"Helen Lovejoy",
	"Reverend Timothy Lovejoy",
	"Coach Lugash",
	"Luigi",
	"Lurleen Lumpkin",
	"Otto Mann",
	"Captain Horatio McCallister",
	"Roger Meyers, Jr.",
	"Troy McClure",
	"Hans Moleman",
	"Dr. Marvin Monroe",
	"Nelson Muntz",
	"Captain Lance Murdock",
	"Bleeding Gums Murphy",
	"Lindsey Naegle",
	"Apu Nahasapeemapetilon",
	"Manjula Nahasapeemapetilon",
	"Sanjay Nahasapeemapetilon",
	"Old Barber",
	"Old Jewish Man",
	"Patches Violet",
	"Poor Violet",
	"Arnie Pye",
	"Poochie",
	"Herbert Powell",
	"Janey Powell",
	"Lois Pennycandy",
	"Ruth Powers",
	"Martin Prince",
	"Dr. J. Loren Pryor",
	"Mayor Diamond Joe Quimby",
	"Radioactive Man",
	"The Rich Texan",
	"Richard",
	"Dr. Nick Riviera",
	"Santas Little Helper",
	"Sherri",
	"Terri",
	"Dave Shutton",
	"Sideshow Bob",
	"Sideshow Mel",
	"Grampa Abraham Simpson",
	"Amber Simpson",
	"Mona Simpson",
	"Agnes Skinner",
	"Principal Seymour Skinner",
	"Waylon Smithers",
	"Snake Jailbird",
	"Snowball",
	"Judge Roy Snyder",
	"Jebediah Springfield",
	"Cletus Spuckler",
	"Brandine Spuckler",
	"Squeaky-Voiced Teen",
	"Moe Szyslak",
	"Drederick Tatum",
	"Allison Taylor",
	"Mr. Teeny",
	"Cecil Terwilliger",
	"Johnny Tightlips",
	"Üter",
	"Kirk Van Houten",
	"Luann Van Houten",
	"Milhouse Van Houten",
	"Dr. Velimirovic",
	"Chief Clancy Wiggum",
	"Ralph Wiggum",
	"Sarah Wiggum",
	"Groundskeeper Willie",
	"Wiseguy",
	"Rainier Wolfcastle",
	"Yes Guy",
	"Artie Ziff",
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static LOCATIONS: [&'static str; 43] = [
	"Springfield",
	"Evergreen Terrace",
	"Springfield Nuclear Power Plant",
	"Kwik-E-Mart",
	"The Androids Dungeon & Baseball Card Shop",
	"Barneys Bowl-A-Rama",
	"Costingtons",
	"KBBL Broadcasting",
	"King Toots",
	"The Leftorium",
	"Noiseland Video Arcade",
	"Sprawl-Mart",
	"Springfield Mall",
	"Stoners Pot Palace",
	"Try-N-Save",
	"Jakes Unisex Hairplace",
	"The Gilded Truffle",
	"Moes Tavern",
	"Krusty Burger",
	"Lard Lad Donuts",
	"Luigis",
	"The Frying Dutchman",
	"The Singing Sirloin",
	"The Happy Sumo",
	"The Java Server",
	"Pimento Grove",
	"Springfield Elementary School",
	"West Springfield Elementary School",
	"Springfield Preparatory School",
	"Springfield High School",
	"Krustylu Studios",
	"Sleep Eazy Motel",
	"Springfield Retirement Castle",
	"The Springfield City Hall",
	"Springfield Courthouse",
	"Five Corners",
	"Krustyland",
	"Shelbyville",
	"Capital City",
	"Brockway",
	"Ogdenville",
	"North Haverbrook",
	"Cypress Creek",
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 19] = [
    "Marriage is like a coffin and each kid is another nail.",
    "It takes two to lie: one to lie and one to listen.",
    "Life is just one crushing defeat after another until you just wish Flanders was dead.",
    "You tried your best and you failed miserably. The lesson is: Never try.",
    "If you pray to the wrong god, you might just make the right one madder and madder.",
    "Kill my boss? Do I dare live out the American dream?",
    "Im not normally a praying man, but if youre up there, please save me, Superman!",
    "Doh!",
    "Thats it! You people have stood in my way long enough. Im going to clown college!",
    "Son, if you really want something in this life, you have to work for it. Now quiet! Theyre about to announce the lottery numbers.",
    "Whats the point of going out? Were just gonna wind up back home anyway.",
    "Cheating is the gift man gives himself.",
    "Books are useless! I only ever read one book, To Kill A Mockingbird, and it gave me absolutely no insight on how to kill mockingbirds!",
    "Sorry, Mom, the mob has spoken.",
    "Go out on a Tuesday? Who am I, Charlie Sheen?",
    "To alcohol! The cause of, and solution to, all of lifes problems.",
    "Trust me, Bart, its better to walk in on both your parents than on just one of them.",
    "Oh, loneliness and cheeseburgers are a dangerous mix.",
    "When will I learn? The answers to lifes problems arent at the bottom of a bottle, theyre on TV!"
];
static QUOTES_LEN: usize = QUOTES.len();

static EPISODE_TITLES: [&'static str; 666] = [
	"Simpsons Roasting on an Open Fire",
	"Bart the Genius",
	"Homers Odyssey",
	"Theres No Disgrace Like Home",
	"Bart the General",
	"Moaning Lisa",
	"The Call of the Simpsons",
	"The Telltale Head",
	"Life on the Fast Lane",
	"Homers Night Out",
	"The Crepes of Wrath",
	"Krusty Gets Busted",
	"Some Enchanted Evening",
	"Bart Gets an F",
	"Simpson and Delilah",
	"Treehouse of Horror",
	"Two Cars in Every Garage and Three Eyes on Every Fish",
	"Dancin Homer",
	"Dead Putting Society",
	"Bart vs. Thanksgiving",
	"Bart the Daredevil",
	"Itchy & Scratchy & Marge",
	"Bart Gets Hit by a Car",
	"One Fish, Two Fish, Blowfish, Blue Fish",
	"The Way We Was",
	"Homer vs. Lisa and the 8th Commandment",
	"Principal Charming",
	"Oh Brother, Where Art Thou?",
	"Barts Dog Gets an F",
	"Old Money",
	"Brush with Greatness",
	"Lisas Substitute",
	"The War of the Simpsons",
	"Three Men and a Comic Book",
	"Blood Feud",
	"Stark Raving Dad",
	"Mr. Lisa Goes to Washington",
	"When Flanders Failed",
	"Bart the Murderer",
	"Homer Defined",
	"Like Father, Like Clown",
	"Treehouse of Horror II",
	"Lisas Pony",
	"Saturdays of Thunder",
	"Flaming Moes",
	"Burns Verkaufen der Kraftwerk",
	"I Married Marge",
	"Radio Bart",
	"Lisa the Greek",
	"Homer Alone",
	"Bart the Lover",
	"Homer at the Bat",
	"Separate Vocations",
	"Dog of Death",
	"Colonel Homer",
	"Black Widower",
	"The Otto Show",
	"Barts Friend Falls in Love",
	"Brother, Can You Spare Two Dimes?",
	"Kamp Krusty",
	"A Streetcar Named Marge",
	"Homer the Heretic",
	"Lisa the Beauty Queen",
	"Treehouse of Horror III",
	"Itchy & Scratchy: The Movie",
	"Marge Gets a Job",
	"New Kid on the Block",
	"Mr. Plow",
	"Lisas First Word",
	"Homers Triple Bypass",
	"Marge vs. the Monorail",
	"Selmas Choice",
	"Brother from the Same Planet",
	"I Love Lisa",
	"Duffless",
	"Last Exit to Springfield",
	"So Its Come to This: A Simpsons Clip Show",
	"The Front",
	"Whacking Day",
	"Marge in Chains",
	"Krusty Gets Kancelled",
	"Homers Barbershop Quartet",
	"Cape Feare",
	"Homer Goes to College",
	"Rosebud",
	"Treehouse of Horror IV",
	"Marge on the Lam",
	"Barts Inner Child",
	"Boy-Scoutz n the Hood",
	"The Last Temptation of Homer",
	"$pringfield (or, How I Learned to Stop Worrying and Love Legalized Gambling)",
	"Homer the Vigilante",
	"Bart Gets Famous",
	"Homer and Apu",
	"Lisa vs. Malibu Stacy",
	"Deep Space Homer",
	"Homer Loves Flanders",
	"Bart Gets an Elephant",
	"Burns Heir",
	"Sweet Seymour Skinners Baadasssss Song",
	"The Boy Who Knew Too Much",
	"Lady Bouviers Lover",
	"Secrets of a Successful Marriage",
	"Bart of Darkness",
	"Lisas Rival",
	"Another Simpsons Clip Show",
	"Itchy & Scratchy Land",
	"Sideshow Bob Roberts",
	"Treehouse of Horror V",
	"Barts Girlfriend",
	"Lisa on Ice",
	"Homer Badman",
	"Grampa vs. Sexual Inadequacy",
	"Fear of Flying",
	"Homer the Great",
	"And Maggie Makes Three",
	"Barts Comet",
	"Homie the Clown",
	"Bart vs. Australia",
	"Homer vs. Patty and Selma",
	"A Star Is Burns",
	"Lisas Wedding",
	"Two Dozen and One Greyhounds",
	"The PTA Disbands",
	"Round Springfield",
	"The Springfield Connection",
	"Lemon of Troy",
	"Who Shot Mr. Burns? (Part One)",
	"Who Shot Mr. Burns? (Part Two)",
	"Radioactive Man",
	"Home Sweet Homediddly-Dum-Doodily",
	"Bart Sells His Soul",
	"Lisa the Vegetarian",
	"Treehouse of Horror VI",
	"King-Size Homer",
	"Mother Simpson",
	"Sideshow Bobs Last Gleaming",
	"The Simpsons 138th Episode Spectacular",
	"Marge Be Not Proud",
	"Team Homer",
	"Two Bad Neighbors",
	"Scenes from the Class Struggle in Springfield",
	"Bart the Fink",
	"Lisa the Iconoclast",
	"Homer the Smithers",
	"The Day the Violence Died",
	"A Fish Called Selma",
	"Bart on the Road",
	"22 Short Films About Springfield",
	"Raging Abe Simpson and His Grumbling Grandson in The Curse of the Flying Hellfish",
	"Much Apu About Nothing",
	"Homerpalooza",
	"Summer of 4 Ft. 2",
	"Treehouse of Horror VII",
	"You Only Move Twice",
	"The Homer They Fall",
	"Burns, Baby Burns",
	"Bart After Dark",
	"A Milhouse Divided",
	"Lisas Date with Density",
	"Hurricane Neddy",
	"El Viaje Misterioso de Nuestro Jomer (The Mysterious Voyage of Homer)",
	"The Springfield Files",
	"The Twisted World of Marge Simpson",
	"Mountain of Madness",
	"Simpsoncalifragilisticexpiala(Annoyed Grunt)cious",
	"The Itchy & Scratchy & Poochie Show",
	"Homers Phobia",
	"Brother from Another Series",
	"My Sister, My Sitter",
	"Homer vs. the Eighteenth Amendment",
	"Grade School Confidential",
	"The Canine Mutiny",
	"The Old Man and the Lisa",
	"In Marge We Trust",
	"Homers Enemy",
	"The Simpsons Spin-Off Showcase",
	"The Secret War of Lisa Simpson",
	"The City of New York vs. Homer Simpson",
	"The Principal and the Pauper",
	"Lisas Sax",
	"Treehouse of Horror VIII",
	"The Cartridge Family",
	"Bart Star",
	"The Two Mrs. Nahasapeemapetilons",
	"Lisa the Skeptic",
	"Realty Bites",
	"Miracle on Evergreen Terrace",
	"All Singing, All Dancing",
	"Bart Carny",
	"The Joy of Sect",
	"Das Bus",
	"The Last Temptation of Krust",
	"Dumbbell Indemnity",
	"Lisa the Simpson",
	"This Little Wiggy",
	"Simpson Tide",
	"The Trouble with Trillions",
	"Girly Edition",
	"Trash of the Titans",
	"King of the Hill",
	"Lost Our Lisa",
	"Natural Born Kissers",
	"Lard of the Dance",
	"The Wizard of Evergreen Terrace",
	"Bart the Mother",
	"Treehouse of Horror IX",
	"When You Dish Upon a Star",
	"Doh-in in the Wind",
	"Lisa Gets an A",
	"Homer Simpson in: Kidney Trouble",
	"Mayored to the Mob",
	"Viva Ned Flanders",
	"Wild Barts Cant Be Broken",
	"Sunday, Cruddy Sunday",
	"Homer to the Max",
	"Im with Cupid",
	"Marge Simpson in: Screaming Yellow Honkers",
	"Make Room for Lisa",
	"Maximum Homerdrive",
	"Simpsons Bible Stories",
	"Mom and Pop Art",
	"The Old Man and the C Student",
	"Monty Cant Buy Me Love",
	"They Saved Lisas Brain",
	"Thirty Minutes over Tokyo",
	"Beyond Blunderdome",
	"Brothers Little Helper",
	"Guess Whos Coming to Criticize Dinner?",
	"Treehouse of Horror X",
	"E-I-E-I-(Annoyed Grunt)",
	"Hello Gutter, Hello Fadder",
	"Eight Misbehavin",
	"Take My Wife, Sleaze",
	"Grift of the Magi",
	"Little Big Mom",
	"Faith Off",
	"The Mansion Family",
	"Saddlesore Galactica",
	"Alone Again, Natura-Diddily",
	"Missionary: Impossible",
	"Pygmoelian",
	"Bart to the Future",
	"Days of Wine and Dohses",
	"Kill the Alligator and Run",
	"Last Tap Dance in Springfield",
	"Its a Mad, Mad, Mad, Mad Marge",
	"Behind the Laughter",
	"Treehouse of Horror XI",
	"A Tale of Two Springfields",
	"Insane Clown Poppy",
	"Lisa the Tree Hugger",
	"Homer vs. Dignity",
	"The Computer Wore Menace Shoes",
	"The Great Money Caper",
	"Skinners Sense of Snow",
	"HOMR",
	"Pokey Mom",
	"Worst Episode Ever",
	"Tennis the Menace",
	"Day of the Jackanapes",
	"New Kids on the Blecch",
	"Hungry, Hungry Homer",
	"Bye Bye Nerdie",
	"Simpson Safari",
	"Trilogy of Error",
	"Im Goin to Praiseland",
	"Children of a Lesser Clod",
	"Simpsons Tall Tales",
	"Treehouse of Horror XII",
	"The Parent Rap",
	"Homer the Moe",
	"A Hunka Hunka Burns in Love",
	"The Blunder Years",
	"She of Little Faith",
	"Brawl in the Family",
	"Sweets and Sour Marge",
	"Jaws Wired Shut",
	"Half-Decent Proposal",
	"The Bart Wants What It Wants",
	"The Lastest Gun in the West",
	"The Old Man and the Key",
	"Tales from the Public Domain",
	"Blame It on Lisa",
	"Weekend at Burnsies",
	"Gump Roast",
	"I Am Furious (Yellow)",
	"The Sweetest Apu",
	"Little Girl in the Big Ten",
	"The Frying Game",
	"Poppas Got a Brand New Badge",
	"Treehouse of Horror XIII",
	"How I Spent My Strummer Vacation",
	"Bart vs. Lisa vs. the Third Grade",
	"Large Marge",
	"Helter Shelter",
	"The Great Louse Detective",
	"Special Edna",
	"The Dad Who Knew Too Little",
	"The Strong Arms of the Ma",
	"Pray Anything",
	"Barting Over",
	"Im Spelling as Fast as I Can",
	"A Star Is Born Again",
	"Mr. Spritz Goes to Washington",
	"C.E.Doh",
	"Scuse Me While I Miss the Sky",
	"Three Gays of the Condo",
	"Dude, Wheres My Ranch?",
	"Old Yeller-Belly",
	"Brake My Wife, Please",
	"The Bart of War",
	"Moe Baby Blues",
	"Treehouse of Horror XIV",
	"My Mother the Carjacker",
	"The President Wore Pearls",
	"The Regina Monologues",
	"The Fat and the Furriest",
	"Today I Am a Clown",
	"Tis the Fifteenth Season",
	"Marge vs. Singles, Seniors, Childless Couples and Teens and Gays",
	"I, (Annoyed Grunt)-bot",
	"Diatribe of a Mad Housewife",
	"Margical History Tour",
	"Milhouse Doesnt Live Here Anymore",
	"Smart & Smarter",
	"The Ziff Who Came to Dinner",
	"Co-Dependents Day",
	"The Wandering Juvie",
	"My Big Fat Geek Wedding",
	"Catch Em If You Can",
	"Simple Simpson",
	"The Way We Werent",
	"Bart-Mangled Banner",
	"Fraudcast News",
	"Treehouse of Horror XV",
	"Alls Fair in Oven War",
	"Sleeping with the Enemy",
	"She Used to Be My Girl",
	"Fat Man and Little Boy",
	"Midnight Rx",
	"Mommie Beerest",
	"Homer and Neds Hail Mary Pass",
	"Pranksta Rap",
	"Theres Something About Marrying",
	"On a Clear Day I Cant See My Sister",
	"Goo Goo Gai Pan",
	"Mobile Homer",
	"The Seven-Beer Snitch",
	"Future-Drama",
	"Dont Fear the Roofer",
	"The Heartbroke Kid",
	"A Star Is Torn",
	"Thank God Its Doomsday",
	"Home Away from Homer",
	"The Father, the Son, and the Holy Guest Star",
	"The Bonfire of the Manatees",
	"The Girl Who Slept Too Little",
	"Milhouse of Sand and Fog",
	"Treehouse of Horror XVI",
	"Marges Son Poisoning",
	"See Homer Run",
	"The Last of the Red Hat Mamas",
	"The Italian Bob",
	"Simpsons Christmas Stories",
	"Homers Paternity Coot",
	"Were on the Road to Dohwhere",
	"My Fair Laddy",
	"The Seemingly Never-Ending Story",
	"Bart Has Two Mommies",
	"Homer Simpson, This Is Your Wife",
	"Million Dollar Abie",
	"Kiss Kiss, Bang Bangalore",
	"The Wettest Stories Ever Told",
	"Girls Just Want to Have Sums",
	"Regarding Margie",
	"The Monkey Suit",
	"Marge and Homer Turn a Couple Play",
	"The Mook, the Chef, the Wife and Her Homer",
	"Jazzy and the Pussycats",
	"Please Homer, Dont Hammer Em",
	"Treehouse of Horror XVII",
	"G.I. (Annoyed Grunt)",
	"MoeNa Lisa",
	"Ice Cream of Margie (with the Light Blue Hair)",
	"The Haw-Hawed Couple",
	"Kill Gil, Volumes I & II",
	"The Wife Aquatic",
	"Revenge Is a Dish Best Served Three Times",
	"Little Big Girl",
	"Springfield Up",
	"Yokel Chords",
	"Rome-Old and Juli-Eh",
	"Homerazzi",
	"Marge Gamer",
	"The Boys of Bummer",
	"Crook and Ladder",
	"Stop! Or My Dog Will Shoot",
	"24 Minutes",
	"You Kent Always Say What You Want",
	"He Loves to Fly and He Dohs",
	"The Homer of Seville",
	"Midnight Towboy",
	"I Dont Wanna Know Why the Caged Bird Sings",
	"Treehouse of Horror XVIII",
	"Little Orphan Millie",
	"Husbands and Knives",
	"Funeral for a Fiend",
	"Eternal Moonshine of the Simpson Mind",
	"E Pluribus Wiggum",
	"That 90s Show",
	"Love, Springfieldian Style",
	"The Debarted",
	"Dial N for Nerder",
	"Smoke on the Daughter",
	"Papa Dont Leech",
	"Apocalypse Cow",
	"Any Given Sundance",
	"Mona Leaves-a",
	"All About Lisa",
	"Sex, Pies and Idiot Scrapes",
	"Lost Verizon",
	"Double, Double, Boy in Trouble",
	"Treehouse of Horror XIX",
	"Dangerous Curves",
	"Homer and Lisa Exchange Cross Words",
	"MyPods and Boomsticks",
	"The Burns and the Bees",
	"Lisa the Drama Queen",
	"Take My Life, Please",
	"How the Test Was Won",
	"No Loan Again, Naturally",
	"Gone Maggie Gone",
	"In the Name of the Grandfather",
	"Wedding for Disaster",
	"Eeny Teeny Maya Moe",
	"The Good, the Sad and the Drugly",
	"Father Knows Worst",
	"Waverly Hills, 9-0-2-1-Doh",
	"Four Great Women and a Manicure",
	"Coming to Homerica",
	"Homer the Whopper",
	"Bart Gets a Z",
	"The Great Wife Hope",
	"Treehouse of Horror XX",
	"The Devil Wears Nada",
	"Pranks and Greens",
	"Rednecks and Broomsticks",
	"O Brother, Where Bart Thou?",
	"Thursdays with Abie",
	"Once Upon a Time in Springfield",
	"Million Dollar Maybe",
	"Boy Meets Curl",
	"The Color Yellow",
	"Postcards from the Wedge",
	"Stealing First Base",
	"The Greatest Story Ever Dohed",
	"American History X-cellent",
	"Chief of Hearts",
	"The Squirt and the Whale",
	"To Surveil with Love",
	"Moe Letter Blues",
	"The Bob Next Door",
	"Judge Me Tender",
	"Elementary School Musical",
	"Loan-a Lisa",
	"MoneyBart",
	"Treehouse of Horror XXI",
	"Lisa Simpson, This Isnt Your Life",
	"The Fool Monty",
	"How Munched is That Birdie in the Window?",
	"The Fight Before Christmas",
	"Donnie Fatso",
	"Moms Id Like to Forget",
	"Flaming Moe",
	"Homer the Father",
	"The Blue and the Gray",
	"Angry Dad: The Movie",
	"The Scorpions Tale",
	"A Midsummers Nice Dream",
	"Love Is a Many Strangled Thing",
	"The Great Simpsina",
	"The Real Housewives of Fat Tony",
	"Homer Scissorhands",
	"500 Keys",
	"The Ned-Liest Catch",
	"The Falcon and the Dohman",
	"Bart Stops to Smell the Roosevelts",
	"Treehouse of Horror XXII",
	"Replaceable You",
	"The Food Wife",
	"The Book Job",
	"The Man in the Blue Flannel Pants",
	"The Ten-Per-Cent Solution",
	"Holidays of Future Passed",
	"Politically Inept, with Homer Simpson",
	"The Doh-cial Network",
	"Moe Goes from Rags to Riches",
	"The Daughter Also Rises",
	"At Long Last Leave",
	"Exit Through the Kwik-E-Mart",
	"How I Wet Your Mother",
	"Them, Robot",
	"Beware My Cheating Bart",
	"A Totally Fun Thing That Bart Will Never Do Again",
	"The Spy Who Learned Me",
	"Ned n Ednas Blend Agenda",
	"Lisa Goes Gaga",
	"Moonshine River",
	"Treehouse of Horror XXIII",
	"Adventures in Baby-Getting",
	"Gone Abie Gone",
	"Penny-Wiseguys",
	"A Tree Grows in Springfield",
	"The Day the Earth Stood Cool",
	"To Cur with Love",
	"Homer Goes to Prep School",
	"A Test Before Trying",
	"The Changing of the Guardian",
	"Love Is a Many-Splintered Thing",
	"Hardly Kirk-ing",
	"Gorgeous Grampa",
	"Black Eyed, Please",
	"Dark Knight Court",
	"What Animated Women Want",
	"Pulpit Friction",
	"Whiskey Business",
	"The Fabulous Faker Boy",
	"The Saga of Carl",
	"Dangers on a Train",
	"Homerland",
	"Treehouse of Horror XXIV",
	"Four Regrettings and a Funeral",
	"YOLO",
	"Labor Pains",
	"The Kid Is All Right",
	"Yellow Subterfuge",
	"White Christmas Blues",
	"Steal This Episode",
	"Married to the Blob",
	"Specs and the City",
	"Diggs",
	"The Man Who Grew Too Much",
	"The Winter of His Content",
	"The War of Art",
	"You Dont Have to Live Like a Referee",
	"Luca$",
	"Days of Future Future",
	"What to Expect When Barts Expecting",
	"Brick Like Me",
	"Pay Pal",
	"The Yellow Badge of Cowardge",
	"Clown in the Dumps",
	"The Wreck of the Relationship",
	"Super Franchise Me",
	"Treehouse of Horror XXV",
	"Opposites A-Frack",
	"Simpsorama",
	"Blazed and Confused",
	"Covercraft",
	"I Wont Be Home for Christmas",
	"The Man Who Came to Be Dinner",
	"Barts New Friend",
	"The Musk Who Fell to Earth",
	"Walking Big & Tall",
	"My Fare Lady",
	"The Princess Guide",
	"Sky Police",
	"Waiting for Duffman",
	"Peeping Mom",
	"The Kids Are All Fight",
	"Lets Go Fly a Coot",
	"Bull-E",
	"Mathletes Feat",
	"Every Mans Dream",
	"Cue Detective",
	"Puffless",
	"Halloween of Horror",
	"Treehouse of Horror XXVI",
	"Friend with Benefit",
	"Lisa with an S",
	"Paths of Glory",
	"Barthood",
	"The Girl Code",
	"Teenage Mutant Milk-Caused Hurdles",
	"Much Apu About Something",
	"Love Is in the N2-O2-Ar-CO2-Ne-He-CH4",
	"Gal of Constant Sorrow",
	"Lisa the Veterinarian",
	"The Marge-ian Chronicles",
	"The Burns Cage",
	"How Lisa Got Her Marge Back",
	"Fland Canyon",
	"To Courier with Love",
	"Simprovised",
	"Orange Is the New Yellow",
	"Monty Burns Fleeing Circus",
	"Friends and Family",
	"The Town",
	"Treehouse of Horror XXVII",
	"Trust but Clarify",
	"There Will Be Buds",
	"Havana Wild Weekend",
	"Dad Behavior",
	"The Last Traction Hero",
	"The Nightmare After Krustmas",
	"Pork and Burns",
	"The Great Phatsby Parts 1 & 2",
	"Fatzcarraldo",
	"The Cad and the Hat",
	"Kamp Krustier",
	"22 for 30",
	"A Fathers Watch",
	"The Caper Chase",
	"Looking for Mr. Goodbart",
	"Moho House",
	"Dogtown",
	"The Serfsons",
	"Springfield Splendor",
	"Whistlers Father",
	"Treehouse of Horror XXVIII",
	"Grampy Can Ya Hear Me",
	"The Old Blue Mayor She Aint What She Used to Be",
	"Singin in the Lane",
	"Mr. Lisas Opus",
	"Gone Boy",
	"Haw-Haw Land",
	"Frink Gets Testy",
	"Homer Is Where the Art Isnt",
	"3 Scenes Plus a Tag from a Marriage",
	"Fears of a Clown",
	"No Good Read Goes Unpunished",
	"King Leer",
	"Lisa Gets the Blues",
	"Forgive and Regret",
	"Left Behind",
	"Throw Grampa from the Dane",
	"Flanders Ladder",
	"Barts Not Dead",
	"Heartbreak Hotel",
	"My Way or the Highway to Heaven",
	"Treehouse of Horror XXIX",
	"Baby You Cant Drive My Car",
	"From Russia Without Love",
	"Werking Mom",
	"Krusty the Clown",
	"Daddicus Finch",
	"Tis the 30th Season",
	"Mad About the Toy",
	"The Girl on the Bus",
	"Im Dancing as Fat as I Can",
	"The Clown Stays in the Picture",
	"101 Mitigations",
	"I Want You (Shes So Heavy)",
	"E My Sports",
	"Bart vs. Itchy & Scratchy",
	"Girls in the Band",
	"Im Just a Girl Who Cant Say Doh",
	"Doh Canada",
	"Woo-Hoo Dunnit?",
	"Crystal Blue-Haired Persuasion",
	"The Winter of Our Monetized Content",
	"Go Big or Go Homer",
	"The Fat Blue Line",
	"Treehouse of Horror XXX",
	"Gorillas on the Mast",
];
static EPISODE_TITLES_LEN: usize = EPISODE_TITLES.len();
