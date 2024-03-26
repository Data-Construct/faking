#[macro_use]
extern crate lazy_static;

pub use self::data::blockchain;
pub use self::data::countries;
pub use self::data::defaults;
pub use self::data::misc;

pub mod locales;

pub mod data {
	pub mod blockchain {
		pub mod bitcoin;
		pub mod cryptocurrency;
		pub mod ethereum;
	}
	pub mod defaults {
		pub mod colors;
		pub mod crypto;
		pub mod date;
		pub mod longitude_latitude;
		pub mod types;
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
		pub mod currencies;
		pub mod demographic;
		pub mod device;
		pub mod fashion;
		pub mod food;
		pub mod greek_philosophers;
		pub mod industry_segments;
		pub mod ipv4;
		pub mod ipv6;
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
}

pub mod utils {
	pub mod seeder;
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		// let result = add(2, 2);
		// assert_eq!(result, 4);
	}

	#[test]
	fn test_greek_philosopher_names() {
		setup_rng();

		use crate::misc::greek_philosophers;
		let name: String = greek_philosophers::greek_philosopher_names();
		let expected: String = "Galen".to_owned();
		assert_eq!(expected, name);
	}

	#[test]
	fn test_greek_philosopher_quotes() {
		setup_rng();

		use crate::misc::greek_philosophers;
		let quote: String = greek_philosophers::greek_philosopher_quotes();
		let expected: String = "Good habits formed at youth make all the difference.".to_owned();
		assert_eq!(expected, quote);
	}

	/*
	 Test variable setup.
	*/
	use std::sync::Once;

	static STARTUP_RUN: Once = Once::new();
	const SEED_VALUE: u64 = 1;

	fn setup_rng() {
		STARTUP_RUN.call_once(|| {
			use crate::utils::seeder;
			seeder::set_seed(SEED_VALUE);
		});
	}
}
