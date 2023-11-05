#[macro_use]
extern crate lazy_static;

pub use self::data::defaults;
pub use self::data::media;
pub use self::data::misc;

pub mod data {
	pub mod defaults {
		pub mod name;
		pub mod types;
		pub mod uuids;
	}
	pub mod misc {
		pub mod addresses;
		pub mod addresses_canada;
		pub mod adjective;
		pub mod ancients;
		pub mod animals;
		pub mod appliances;
		pub mod artists;
		pub mod blood;
		pub mod books;
		pub mod business;
		pub mod chess;
		pub mod codes;
		pub mod commerce;
		pub mod currencies;
		pub mod demographic;
		pub mod device;
		pub mod emails;
		pub mod fashion;
		pub mod food;
		pub mod job;
		pub mod lorem_ipsum;
		pub mod military;
		pub mod nations;
		pub mod programming_languages;
		pub mod relationship;
		pub mod restaurant;
		pub mod space;
		pub mod sports;
		pub mod stripe;
		pub mod subscription;
		pub mod tea;
		pub mod usernames;
	}
	pub mod media {
		pub mod games;
		pub mod mario;
		pub mod movies;
		pub mod pokemon;
		pub mod starwars;
		pub mod starwars_yoda;
	}
	pub mod religion {
		pub mod bible;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		// let result = add(2, 2);
		// assert_eq!(result, 4);
	}
}
