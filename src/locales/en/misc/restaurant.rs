use crate::utils::seeder;
use wasm_bindgen::prelude::*;
// ask about ### and  ???
// TODO kevinly77: Ask about generating a restuarant object?

#[wasm_bindgen]
pub fn name_prefix() -> String {
	NAME_PREFIX[seeder::gen_range(0..NAME_PREFIX_LEN)].to_string()
}

#[wasm_bindgen]
pub fn name_suffix() -> String {
	NAME_SUFFIX[seeder::gen_range(0..NAME_SUFFIX_LEN)].to_string()
}

#[wasm_bindgen]
pub fn restaurant_full_name() -> String {
	let prefix = name_prefix();
	let suffix = name_suffix();
	return concat_string!(prefix, " ", suffix);
}

#[wasm_bindgen]
pub fn restaurant_type() -> String {
	RESTAURANT_TYPE[seeder::gen_range(0..RESTAURANT_TYPE_LEN)].to_string()
}

#[wasm_bindgen]
pub fn restaurant_description() -> String {
	RESTAURANT_DESCRIPTION[seeder::gen_range(0..RESTAURANT_DESCRIPTION_LEN)].to_string()
}

#[wasm_bindgen]
pub fn restaurant_review() -> String {
	RESTAURANT_REVIEW[seeder::gen_range(0..RESTAURANT_REVIEW_LEN)].to_string()
}

#[wasm_bindgen]
pub fn generate_full_restaurant() -> String {
	let full_name = restaurant_full_name();
	let restaurant_type = restaurant_type();
	let restaurant_description = restaurant_description();
	let restaurant_review = restaurant_review();
	return format!(
		"Name:{}\n Type:{}\n Description:{}\n Review:{}\n",
		full_name, restaurant_type, restaurant_description, restaurant_review
	);
}

static NAME_PREFIX: [&'static str; 23] = [
	"??",
	"???",
	"##",
	"###",
	"####",
	"Belly",
	"Big",
	"Blue Plate",
	"Fast",
	"Fat",
	"Golden",
	"Hungry",
	"Salty",
	"Silver",
	"Smokestack",
	"Spice",
	"Sugar",
	"Sweet",
	"Thirsty",
	"Red",
	"Blue",
	"Green",
	"Orange",
];
static NAME_PREFIX_LEN: usize = NAME_PREFIX.len();

static NAME_SUFFIX: [&'static str; 29] = [
	"Bakery",
	"Bar & Grill",
	"BBQ",
	"Box",
	"Brasserie",
	"Burger",
	"Cafe",
	"Coffee",
	"Creamery",
	"Curry",
	"Deli",
	"Diner",
	"Dragon",
	"Eatery",
	"Eats",
	"Gastropub",
	"Grill",
	"Grill & Tap",
	"House",
	"Juice Bar",
	"King",
	"Kitchen",
	"Pizza",
	"Pub",
	"Shakes",
	"Spoon",
	"Steakhouse",
	"Subs",
	"Sushi",
];
static NAME_SUFFIX_LEN: usize = NAME_SUFFIX.len();

static RESTAURANT_TYPE: [&'static str; 36] = [
	"African",
	"American (New)",
	"American (Traditional)",
	"Argentinian",
	"Asian",
	"Bakery",
	"Bar",
	"Brazilian",
	"Burgers",
	"Caribbean",
	"Chinese",
	"Comfort Food",
	"Desserts",
	"Ethiopean",
	"European",
	"French",
	"German",
	"Greek",
	"Healthy",
	"Ice Cream",
	"Indian",
	"Italian",
	"Japanese",
	"Juice & Smoothies",
	"Korean",
	"Mexican",
	"Pizza",
	"Ramen",
	"Sandwiches",
	"Senegalese",
	"Sushi",
	"Tex Mex",
	"Thai",
	"Vegan",
	"Vegetarian",
	"Vietnamese",
];
static RESTAURANT_TYPE_LEN: usize = RESTAURANT_TYPE.len();

static RESTAURANT_DESCRIPTION: [&'static str; 14] = [
    "To ensure that each guest receives prompt, professional, friendly and courteous service. To maintain a clean, comfortable and well maintained premises for our guests and staff. To provide at a fair price – nutritional, well-prepared meals – using only quality ingredients. To ensure that all guests and staff are treated with the respect and dignity they deserve. To thank each guest for the opportunity to serve them. By maintaining these objectives we shall be assured of a fair profit that will allow us to contribute to the community we serve.",
    "To sell delicious and remarkable food and drinks. That the food and drink we sell meets the highest standards of quality, freshness and seasonality and combines both modern-creative and traditional southern styles of cooking. To consistently provide our customers with impeccable service by demonstrating warmth, graciousness, efficiency, knowledge, professionalism and integrity in our work. To have every customer who comes through our doors leave impressed by Maxies and excited to come back again. To create and maintain a restaurant that is comprehensive and exceptional in its attention to every detail of operation. To provide all who work with us a friendly, cooperative and rewarding environment which encourages long- term, satisfying, growth employment. To keep our concept fresh, exciting and on the cutting edge of the hospitality and entertainment industry. To be a giving member of the Ithaca community and to use our restaurant to improve the quality of life in the Finger Lakes region.",
    "We are committed to using the finest ingredients in our recipes. No food leaves our kitchen that we ourselves would not eat.",
    "To achieve and maintain such distinction in food and wine, service, atmosphere and setting that the restaurant gains a first class reputation for gastronomy, gracious and informed hospitality, comfort and beauty which draws new and repeat customers year after year. To achieve the above whilst upholding staff policies and practices which promote a fair and positive working environment. To be aware of and act on our responsibilities as a good corporate citizen to  provide a safe, clean and attractive place for guests to enjoy and for employees to work in - ensure ecologically sound management practices at the restaurant and in our surrounding gardens and woods - undertake meaningful involvement of Restaurant Les Fougères in selected charitable activities in our community and region.",
    "To provide an exceptional dining experience that satisfies our guests grown-up tastes by being a Cut-Above in everything we do.",
    "Our Mission at Dennys is to establish beneficial business relationships with diverse suppliers who share our commitment to customer service, quality and competitive pricing.",
    "Yoshinoya is in business to create the structure and systems needed to allow our customers access to the majority of their away-from-home daily meal requirements on a one-stop-shop basis. All our products shall be of the highest quality and value, be healthy, nutritious and provided with outstanding personal services at the lowest possible prices consistent with a fair return on investment for our shareholders, job enhancementsecurity for our employees and a level of community involvement by everyone connected with our business. All of our products and services shall be delivered consistently and measured one satisfied customer at a time, whether by company-owned or franchised operations, in superior, clean, convenient, fun and friendly neighborhood environments. We pledge to make Yoshinoya the best place to eat and the best place to work.",
    "Culvers Restaurant was founded by the Culver family in 1984, which eventually branched out to more than 300 franchised restaurants all over the US. Culvers is well-known for its ButterBurger, which made the restaurant extremely famous. They also have other items which include salads, sandwiches, desserts, etc.",
    "Our mission is to be a leader in the distribution and merchandising of food, pharmacy, health and personal care items, seasonal merchandise, and related products and services. We place considerable importance on forging strong supplier partnerships. Our suppliers, large or small, local or global, are essential components in accomplishing our mission.",
    "We earn the loyalty of the people we serve by first anticipating, then fulfilling their needs with our superior-quality products, a unique shopping experience, customer-focused service and continuous innovation, while generating long-term profitable growth for our shareholders.",
    "Delhaize Group will achieve leading positions in food retailing in key mature and emerging markets. We accomplish our goal by developing strong regional companies benefiting from and contributing to the Groups strength, expertise and successful practices. Delhaize Group goes to market with a variety of food store formats. The Group is committed to offer a locally differentiated shopping experience to its customers in each of its markets, to deliver superior value and to maintain high social, environmental and ethical standards.",
    "SVIs mission is to deliver quality products at affordable prices to our independent retailers, wholesalers and food service partners around the world by providing international procurement, distribution, marketing and supply chain management.",
    "Our mission has been to help people achieve their health and wellness goals. though weve changed over the years, our values have remained the same.",
    "To deliver an exceptional shopping experience by offering the best service, value, quality, and freshest products while being good stewards of our environment and giving back to the communities we serve."
];
static RESTAURANT_DESCRIPTION_LEN: usize = RESTAURANT_DESCRIPTION.len();

static RESTAURANT_REVIEW: [&'static str; 17] = [
    "For dinner we ordered the shrimp enchiladas, chicken enchiladas, chicken burrito, chimichangas, and steak quesadillas. Everything was so tasty and amazing. I wasnt surprised because the food at the FiDi location is the best so I figured this location would be just as good and it was!!! The enchiladas with the green sauce is to die for. My go to at the FiDi location is usually the chicken enchiladas but I decided to try something new and the shrimp enchiladas did not disappoint.",
    "For dessert, we ordered the chocolate drizzled churros and they were delicious too! They even came with some fresh fruit (blueberries and strawberries).",
    "Overall, the evening was a smash and I am so glad there is a new location closer to my office!!",
    "Brand new. Great design. Odd to hear pop music in a Mexican establishment. Music is a bit loud. It should be background.",
    "The chicken nachos were delicious and the atmosphere was great. The frozen margaritas were a little on the weak side. Would probably return for a work happy hour but was pretty disappointed about the lack of free tequila and beer we read about!",
    "The counter is on the left side, and so is the menu. It can get pretty busy with 30 min wait times. I recommend checking the website and see how busy their store is.",
    "They are way understaffed, where the cashier needs to stop taking orders to help pack to-go orders. The line ends up piling up and people are neglected.  Seen this happen multiple times during lunch hours.",
    "This particular location like the many other restaurants down the block has ample seating and a second floor.",
    "I first heard about this place through Instagram post. The drinks looked creative so I made it a point to give it try while in the city. Located inside American Eagle in Times Square they serve up the coolest non-alcoholic beverages. You have an array of options from tea, soda, coffee, latte, water and more. Theres countertops (with phone chargers) so youre able to stand and enjoy your beverage. This visit I opted for the Pegasus latte which was not only beautiful but tasted just as good as it looked. Great staff and great service. This is a must try if you are in the area. Im sure Ill be back soon!",
    "Great place to stop in from a chaotic Times Square adventure! The coffee is great, the drinks are creative and the staff is very nice and hospitable. Cant wait to stop in again. Without a doubt 5 stars from me!!",
    "Great lattes and cold drinks as well. Nice to see a place carrying local coffee and products in Times Square! Excited to make it a part of my morning commute as it is much less crowded than the chain coffee shops around here.",
    "Fish was high quality but portions were small. $57 for 9 pieces and a hand roll which is fair but on the expensive side.",
    "Ambience was good, service was no nonsense but friendly.",
    "Id have to say that each piece was fresh and had their own slight unique flavor twist to it, pushing the line between traditional edomae and fusion. The Hokkaido uni and the ocean trout topped with cook seaweed was the highlights of the meal. But each piece served was really something special.",
    "Staff was very accommodating but the chef were no nonsense. The ambiance is clean and tranquil which is perfect if youre looking to have a conversation with a date or a friend.",
    "My only critique would be that the rice could of used more vingaer and for them to use real wasabi. Also the variety of fish in stock wasnt a lot but hopefully that can change over time. The cost justify the quality youre getting. A solid 4 stars.",
    "In terms of omakase, they had a few options but the one we chose was the 87 dollar version which include sashimi and sushi."
];
static RESTAURANT_REVIEW_LEN: usize = RESTAURANT_REVIEW.len();