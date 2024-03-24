#[macro_use]
extern crate lazy_static;

pub use self::data::blockchain;
pub use self::data::countries;
pub use self::data::defaults;
pub use self::data::media;
pub use self::data::misc;

pub mod data {
	pub mod blockchain {
		pub mod bitcoin;
		pub mod ethereum;
	}
	pub mod defaults {
		pub mod colors;
		pub mod crypto;
		pub mod emails;
		pub mod longitude_latitude;
		pub mod name;
		pub mod phone_numbers;
		pub mod types;
		pub mod usernames;
		pub mod uuids;
	}
	pub mod countries {
		pub mod nations;

		pub mod usa {
			pub mod addresses;
		}
		pub mod canada {
			pub mod addresses;
		}
	}
	pub mod misc {
		pub mod adjective;
		pub mod agent_bot;
		pub mod ancients;
		pub mod animals;
		pub mod appliances;
		pub mod artists;
		pub mod barcodes;
		pub mod blood;
		pub mod books;
		pub mod business;
		pub mod chess;
		pub mod codes;
		pub mod commerce;
		pub mod compass;
		pub mod cryptocurrency;
		pub mod currencies;
		pub mod date;
		pub mod demographic;
		pub mod device;
		pub mod fashion;
		pub mod food;
		pub mod greek_philosophers;
		pub mod industry_segments;
		pub mod ipv4;
		pub mod ipv6;
		pub mod job;
		pub mod lorem_ipsum;
		pub mod mac_address;
		pub mod marketing;
		pub mod measurements;
		pub mod military;
		pub mod programming_languages;
		pub mod quotes;
		pub mod relationship;
		pub mod restaurant;
		pub mod shakespeare;
		pub mod space;
		pub mod sports;
		pub mod stripe;
		pub mod subscription;
		pub mod tea;
	}
	pub mod media {
		pub mod elderscrolls;
		pub mod friends;
		pub mod games;
		pub mod hp_lovecraft;
		pub mod kpop;
		pub mod lord_of_the_rings;
		pub mod manga;
		pub mod mario;
		pub mod minecraft;
		pub mod movies;
		pub mod one_piece;
		pub mod pokemon;
		pub mod seinfeld;
		pub mod silicon_valley;
		pub mod simpsons;
		pub mod spongebob;
		pub mod starwars;
		pub mod starwars_yoda;
		pub mod studio_ghibli;
		pub mod the_hobbit;
		pub mod tolkein;
	}
	pub mod religion {
		pub mod bible;
	}
}

mod utils {
  pub mod seeder;
}

#[cfg(test)]
mod tests {
  const SEED_VALUE: u64 = 1;

	#[test]
	fn it_works() {
		// let result = add(2, 2);
		// assert_eq!(result, 4);
	}

  #[test]
  fn test_greek_philosopher_names() {
    use crate::utils::seeder;
    seeder::set_seed(SEED_VALUE);
    

    use crate::misc::greek_philosophers;
    let name: String = greek_philosophers::greek_philosopher_names();
    let expected: String = "Galen".to_owned();
    assert_eq!(expected, name);
  }

  #[test]
  fn test_greek_philosopher_quotes() {
    use crate::utils::seeder;
    seeder::set_seed(SEED_VALUE);
    

    use crate::misc::greek_philosophers;
    let quote: String = greek_philosophers::greek_philosopher_quotes();
    let expected: String = "Good habits formed at youth make all the difference.".to_owned();
    assert_eq!(expected, quote);
  }
}
