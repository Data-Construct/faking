use wasm_bindgen::prelude::*;

use super::creature;
use crate::locales::en::location::state;

#[wasm_bindgen(js_name = team_name)]
pub fn name() -> String {
    let return_va = state::state()+ " " +&creature::creature();
    return return_va;
}


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = name();
// 		println!("{}", result);
//         println!("-----------------");
// 	}


	 
// }