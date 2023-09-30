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
		pub mod adjective;
		pub mod animals;
		pub mod business;
		pub mod commerce;
		pub mod currencies;
		pub mod demographic;
		pub mod device;
		pub mod fashion;
		pub mod food;
		pub mod relationship;
		pub mod restaurant;
	}
	pub mod media {
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
