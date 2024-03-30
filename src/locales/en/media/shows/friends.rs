use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn friends_characters() -> String {
	CHARACTERS[seeder::gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn friends_locations() -> String {
	LOCATIONS[seeder::gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn friends_quotes() -> String {
	QUOTES[seeder::gen_range(0..QUOTES_LEN)].to_string()
}

static CHARACTERS: [&'static str; 79] = [
  "Peter Becker",
  "Ben",
  "Chandler Bing",
  "Charles Bing",
  "Erica Bing",
  "Jack Bing",
  "Nora Tyler Bing",
  "Bonnie",
  "Amanda Buffamonteezi",
  "Frank Buffay, Jr.",
  "Frances Buffay",
  "Phoebe Buffay",
  "Ursula Buffay",
  "Joshua Burgin",
  "Richard Burke",
  "Chloe",
  "Will Colbert",
  "Danny",
  "David",
  "Rob Donnan",
  "Doug",
  "Eric",
  "Erica",
  "Erika",
  "Barry Farber",
  "Fun Bobby",
  "Gary",
  "Emma Geller",
  "Jack Geller",
  "Monica Geller",
  "Ross Geller",
  "Missy Goldberg",
  "Janice Goralnik",
  "Leonard Green",
  "Rachel Green",
  "Bitsy Hannigan",
  "Mike Hannigan",
  "Theodore Hannigan",
  "Mr. Heckles",
  "Benjamin Hobart",
  "Joanna",
  "Tag Jones",
  "Julie",
  "Kathy",
  "Kim",
  "Alice Knight",
  "Janine LaCroix",
  "Dr. Ledbetter",
  "Estelle Leonard",
  "Dr. Long",
  "Marcel",
  "Eddie Menuek",
  "Kate Miller",
  "Gavin Mitchell",
  "Molly",
  "Mona",
  "Susie Moss",
  "Parker",
  "Mark Robinson",
  "Roy",
  "Ryan",
  "Sandy",
  "Sophie",
  "Steve",
  "Elizabeth Stevens",
  "Paul Stevens",
  "Stu",
  "Terry",
  "Mr. Treeger",
  "Joey Tribbiani",
  "Ugly Naked Guy",
  "Mr. Waltham",
  "Andrea Waltham",
  "Emily Waltham",
  "Stephen Waltham",
  "Charlie Wheeler",
  "Mr. Zelner",
  "Carol Willick",
  "Miss Chanandler Bong"
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static LOCATIONS: [&'static str; 30] = [
  "Monica's Apartment",
  "Central Perk",
  "Chandler and Joey's Apartment",
  "Monica and Chandler's House",
  "Phoebe's Apartment",
  "Ross' Third Apartment",
  "Javu",
  "Moondance Diner",
  "Allesandro's",
  "Bloomingdale's",
  "Ross' First Apartment",
  "Ross' Second Apartment",
  "Lincoln High School",
  "Joey's Apartment",
  "Monica and Ross' Parents House",
  "Becco",
  "Dot's Spot",
  "Carol and Susan's Apartment",
  "Gina's House",
  "Waltham House",
  "Celestino Custom Tailor",
  "Ernie's",
  "New York University",
  "Silvercup Studios",
  "Marcel's",
  "945 Grove St Apt. 20",
  "Ralph Lauren",
  "New York Museum of Prehistoric History",
 "Days of Our Lives",
  "15 Yemen Road, Yemen"
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 50] = [
  "I can handle this. Handle is my middle name. Actually, handle is the middle of my first name.",
  "Pivot! Pivot! Pivot! Pivot! Pivot!",
  "Shut up! Shut up! Shut up!",
  "Joey doesn't share food!",
  "Guys can fake it? Unbelievable! The one thing that's ours!",
  "If you're going to call me names, I would prefer Ross, the Divorce Force. It's just cooler.",
  "All right, look if you absolutely have to tell her the truth, at least wait until the timing's right. And that's what deathbeds are for.",
  "Your teeth? Yeah, I saw them from outside.",
  "Dear God! This parachute is a knapsack!",
  "No, homo habilis was erect. Australopithecus was never fully erect.",
  "Well, maybe he was nervous.",
  "I'm glad we're having a rehearsal dinner. I rarely practice my meals before I eat.",
  "Until I was 25 I thought the only response to ‘I love you' was ‘Oh crap!'",
  "What's not to like? Custard – good. Jam – good. Meat – good!",
  "No. Interestingly enough, her leaf blower picked up.",
  "You know what's weird? Donald Duck never wore pants. But whenever he's getting out of the shower, he always puts a towel around his waist. I mean, what is that about?",
  "Ross, just for my own peace of mind, you're not married to any more of us are you?",
  "It's a moo point. It's like a cow's opinion; it doesn't matter. It's moo.",
  "First divorce: wife's hidden sexuality, not my fault. Second divorce: said the wrong name at the altar, kind of my fault. Third divorce: they shouldn't let you get married when you're that drunk and have stuff drawn all over your face, Nevada's fault.",
  "It hurts my Joey's apple!",
  "Oh, are you setting Ross up with someone? Does she have a wedding dress?",
  "Your collective dating record reads like a who's who of human crap.",
  "If you don't help me cook I'm going to take a bunch of those hot dogs and make a new appetiser called pigs in Ross.",
  "I'm not so good with the advice. Can I interest you in a sarcastic comment?",
  "No! No, Joey! U-N-I-sex.",
  "Oh look, ugly naked guy is decorating his Christmas tree! Wow, you should see the size of his Christmas balls!",
  "I can't believe my Dad saw us having sex! He didn't make it to one of my piano recitals, but this he sees!",
  "If you want to receive emails about my upcoming shows please give me money so I can buy a computer.",
  "Come on Ross you're a palaeontologist – dig a little deeper.",
  "You can't have S-E-X when your taking care of the B-A-B-I-E.",
  "A hundred million people went to see a movie about what I do. I wonder how many people would go see a movie called Jurassic Parka. No, no, no, a bunch of out-of-control jackets take over an island!",
  "Hey buddy, this is a family place. Put the mouse back in the house.",
  "I'm not someone who goes after a guy five minutes after he's divorced.",
  "We were on a break!",
  "Oh, my God! If you say that one more time, I'm going to break up with you!",
  "How you doin'?",
  "Forty-two to twenty-one! Like the turkey, Ross is done!",
  "SEVEN! SEVEN! SEVEN!",
  "I'm Monica. I'm disgusting. I stalk guys and keep their underpants.",
  "Fine judge all you want but... married a lesbian, left a man at the altar, fell in love with a gay ice dancer, threw a girl's wooden leg in the fire, LIVE IN A BOX.",
  "Welcome to the real world. It sucks. You're gonna love it!",
  "Sure I peed on her. And if I had to, I'd pee on any one of you!",
  "If the homo sapiens were, in fact, HOMO sapiens…is that why they're extinct?",
  "You could not be any more wrong. You could try, but you would not be successful.",
  "You've been BAMBOOZLED!",
  "It was summer… and it was hot. Rachel was there… A lonely grey couch…”OH LOOK!” cried Ned, and then the kingdom was his forever. The End.",
  "Je m'appelle Claude",
  "Raspberries? Good. Ladyfingers? Good. Beef? GOOD!",
  "Could I BE wearing any more clothes?",
  "Oh no, two women love me. They're both gorgeous and sexy. My wallet's too small for my fifties AND MY DIAMOND SHOES ARE TOO TIGHT."
];
static QUOTES_LEN: usize = QUOTES.len();