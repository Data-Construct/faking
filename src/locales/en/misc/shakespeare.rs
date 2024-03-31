use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn shakespeare_hamlet_quotes() -> String {
    HAMLET[seeder::gen_range(0..HAMLET_LEN)].to_string()
}

#[wasm_bindgen]
pub fn shakespeare_as_you_like_it_quotes() -> String {
    AS_YOU_LIKE_IT[seeder::gen_range(0..AS_YOU_LIKE_IT_LEN)].to_string()
}

#[wasm_bindgen]
pub fn shakespeare_king_richard_iii_quotes() -> String {
    KING_RICHARD_III[seeder::gen_range(0..KING_RICHARD_III_LEN)].to_string()
}

#[wasm_bindgen]
pub fn shakespeare_romeo_and_juliet_quotes() -> String {
    ROMEO_AND_JULIET[seeder::gen_range(0..ROMEO_AND_JULIET_LEN)].to_string()
}

static HAMLET: [&'static str; 16] = [
  "To be, or not to be, that is the question",
  "Neither a borrower nor a lender be; For loan oft loses both itself and friend, and borrowing dulls the edge of husbandry.",
  "This above all: to thine own self be true, and it must follow, as the night the day, thou canst not then be false to any man.",
  "Though this be madness, yet there is method int.",
  "That it should come to this!",
  "There is nothing either good or bad, but thinking makes it so.",
  "What a piece of work is man! How noble in reason, how infinite in faculty! In form and moving how express and admirable! In action how like an angel, in apprehension how like a god! The beauty of the world. The paragon of animals.",
  "The lady doth protest too much, methinks.",
  "A little more than kin, and less than kind.",
  "The plays the thing wherein I will catch the conscience of the king.",
  "Brevity is the soul of wit.",
  "Doubt thou the stars are fire, doubt that the sun doth move, doubt truth to be a liar, but never doubt I love.",
  "Rich gifts wax poor when givers prove unkind.",
  "Do you think I am easier to be played on than a pipe?",
  "I will speak daggers to her but use none.",
  "When sorrows come, they come not single spies but in battalions."
];
static HAMLET_LEN: usize = HAMLET.len();

static AS_YOU_LIKE_IT: [&'static str; 8] = [
  "All the worlds a stage, and all the men and women merely players; they have their exits and their entrances, and one man in his time plays many parts, his acts being seven ages.",
  "Can one desire too much of a good thing?",
  "I like this place and willingly could waste my time in it.",
  "But Oh, how bitter a thing it is to look into happiness through another mans eyes.",
  "Blow, blow, thou winter wind, thou art not so unkind as mans ingratitude.",
  "True is it that we have seen better days and have with holy bell been knolled to church, and sat at good mens feasts and wiped our eyes of drops that sacred pity hath engendered.",
  "For ever and a day.",
  "The fool doth think he is wise, but the wise man knows himself to be a fool."
];
static AS_YOU_LIKE_IT_LEN: usize = AS_YOU_LIKE_IT.len();

static KING_RICHARD_III: [&'static str; 8] = [
  "Now is the winter of our discontent made glorious summer by this sun of York",
  "A horse! a horse! my kingdom for a horse!",
  "Conscience is but a word that cowards use, devised at first to keep the strong in awe.",
  "So wise so young, they say, do never live long.",
  "But then I sigh and, with a piece of scripture, tell them that God bids us do good for evil; and thus I clothe my naked villainy with odd old ends stolen out of Holy Writ, and seem a saint when most I play the devil.",
  "An honest tale speeds best, being plainly told.",
  "The kings name is a tower of strength.",
  "The world is grown so bad that wrens make prey where eagles dare not perch."
];
static KING_RICHARD_III_LEN: usize = KING_RICHARD_III.len();

static ROMEO_AND_JULIET: [&'static str; 10] = [
  "O Romeo, Romeo! Wherefore art thou Romeo?",
  "But soft! What light through yonder window breaks? It is the east, and Juliet is the sun.",
  "Good night, good night! Parting is such sweet sorrow, that I shall say good night till it be morrow.",
  "Whats in a name? That which we call a rose by any other name would smell as sweet.",
  "Wisely and slow; they stumble that run fast.",
  "Tempt not a desperate man.",
  "For you and I are past our dancing days.",
  "Oh, she doth teach the torches to burn bright! It seems she hangs upon the cheek of night like a rich jewel in an Ethiopes ear, beauty too rich for use, for earth too dear.",
  "See how she leans her cheek upon her hand. Oh, that I were a glove upon that hand that I might touch that cheek!",
  "Not stepping oer the bounds of modesty."
];
static ROMEO_AND_JULIET_LEN: usize = ROMEO_AND_JULIET.len();