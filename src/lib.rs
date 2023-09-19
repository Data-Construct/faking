pub use self::data::defaults;
pub use self::data::misc;
pub use self::data::media;

pub mod data {
	pub mod defaults {
		pub mod name;
		pub mod types;
		pub mod uuids;
	}
	pub mod misc {
		pub mod animals;
		pub mod business;
	}
	pub mod media {
		pub mod starwars;
		pub mod starwars_yoda;
		pub mod movies;
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
