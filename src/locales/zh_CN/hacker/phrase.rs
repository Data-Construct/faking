use crate::utils::seeder;
use wasm_bindgen::prelude::*;
use super::adjective::adjective;
use super::noun::noun;
use super::verb::verb;
use crate::locales::base::hacker::abbreviation::abbreviation;

#[wasm_bindgen(js_name = zh_CN_hacker_phrase)]
pub fn phrase() -> String {
    let format = seeder::gen_range(0..8);

	match format {
		0 => format!("倘若我们{}{}, 我们就可以通过{}{}{}获得{}{}!", verb(), noun(), adjective(), abbreviation(), noun(), abbreviation(), noun()),
        1 => format!("我们需要{}{}{}{}!", verb(), adjective(), abbreviation(), noun()),
        2 => format!("尝试{}{}{}, 也许会{}{}{}!", verb(), abbreviation(), noun(), verb(), adjective(), noun()),
        3 => format!("在没有{}{}{}{}的情况下，你不能{}{}!", verb(), adjective(), abbreviation(), noun(), verb(), noun()),
        4 => format!("使用{}{}{}, 然后你就能{}{}{}!", adjective(), abbreviation(), noun(), verb(), adjective(), noun()),
        5 => format!("{}{}已关闭，因为{}{}所以我们能{}{}{}!", abbreviation(), noun(), adjective(), noun(), verb(), abbreviation(), noun()),
        6 => format!("{}{}是无济于事的，我们需要{}{}{}{}!", verb(), noun(), verb(), adjective(), abbreviation(), noun()),
        7 => format!("我将{}{}{}{}，那是应该{}{}{}!", verb(), adjective(), abbreviation(), noun(), noun(), abbreviation(), noun()),
        _ => "".to_string(),
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = phrase();
// 		print!("-------");
        
//         println!("{}", result);
// 	}


// }