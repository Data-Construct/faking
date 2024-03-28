#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate concat_string;

pub use self::data::blockchain;
pub use self::data::countries;
pub use self::data::defaults;

pub mod locales;


pub mod data {
	pub mod api;

	pub mod blockchain {
		pub mod bitcoin;
		pub mod cryptocurrency;
		pub mod ethereum;
	}

	pub mod defaults {
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

	pub mod it;
}

pub mod utils {
	pub mod seeder;
}

// #[cfg(test)]
// mod tests {
// 	#[test]
// 	fn it_works() {
// 		// let result = add(2, 2);
// 		// assert_eq!(result, 4);
// 	}

// 	#[test]
// 	fn test_greek_philosopher_names() {
// 		setup_rng();

// 		use crate::misc::greek_philosophers;
// 		let name: String = greek_philosophers::greek_philosopher_names();
// 		let expected: String = "Galen".to_owned();
// 		assert_eq!(expected, name);
// 	}

// 	#[test]
// 	fn test_greek_philosopher_quotes() {
// 		setup_rng();

// 		use crate::misc::greek_philosophers;
// 		let quote: String = greek_philosophers::greek_philosopher_quotes();
// 		let expected: String = "Good habits formed at youth make all the difference.".to_owned();
// 		assert_eq!(expected, quote);
// 	}

// 	/*
// 	 Test variable setup.
// 	*/
// 	use std::sync::Once;

// 	static STARTUP_RUN: Once = Once::new();
// 	const SEED_VALUE: u64 = 1;

// 	fn setup_rng() {
// 		STARTUP_RUN.call_once(|| {
// 			use crate::utils::seeder;
// 			seeder::set_seed(SEED_VALUE);
// 		});
// 	}
// }
