#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate concat_string;

extern crate url;

pub mod locales;
pub mod data;

pub mod utils {
	pub mod seeder;
}

// #[cfg(test)]
// mod tests {
//   use crate::{utils::seeder};

// 	#[test]
// 	fn it_works() {
//     // do something
// 	}
// }