#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate concat_string;

pub use self::data::blockchain;
pub use self::data::countries;
pub use self::data::datetime;
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

  pub mod datetime;

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
//   use crate::{defaults::uuids::{uid_v4}, utils::seeder};

// 	#[test]
// 	fn it_works() {
//     // do something
// 	}
// }