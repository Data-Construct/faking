use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = he_color_human)]
pub fn human() -> String {
	HE_HUMAN[seeder::gen_range(0..HE_HUMAN_LEN)].to_string()
}

static HE_HUMAN: [&'static str; 35] = [
    "אדום",
    "ירוק",
    "כחול",
    "צהוב",
    "סגול",
    "ירוק מנטה",
    "ירוק כחלחל",
    "לבן",
    "שחור",
    "כתום",
    "ורוד",
    "אפור",
    "חום ערמוני",
    "טורקיז",
    "שזוף",
    "שמים כחולים",
    "סלמון",
    "שזיף",
    "סחלב",
    "זית",
    "מגנטה",
    "ליים",
    "שנהב",
    "אינדיגו",
    "זהב",
    "ורוד פוקסיה",
    "צבע תכלת",
    "לבנדר",
    "כסף",
    "בורדו",
    "בז",
    "חאקי",
    "ברונזה",
    "ארד",
    "ערמון",
];
static HE_HUMAN_LEN: usize = HE_HUMAN.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = human();
// 		assert!(HE_HUMAN.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
//         print!("--------------");// This assumes you can iterate over EN_NAME, which would require it to be a Vec or a slice
//         println!();
//         println!("{}", result);
//         print!("--------------");// This assumes you can iterate over EN_NAME, which would require it to be a Vec or a slice
        
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 35;
// 		assert!(HE_HUMAN.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
