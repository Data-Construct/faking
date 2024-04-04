use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn genre() -> String {
	HE_GENRE[seeder::gen_range(0..HE_GENRE_LEN)].to_string()
}

static HE_GENRE: [&'static str; 20] = [
    "רוק",
    "רוק מטאלי",
    "פופ",
    "אלקטרוני",
    "מוזיקת עם",
    "מוזיקת עולם",
    "קאנטרי",
    "ג'אז",
    "פאנק",
    "נשמה",
    "היפ הופ",
    "קלאסית",
    "לטינית",
    "רגאיי",
    "במה ומסך",
    "בלוז",
    "לא מוסיקה",
    "ראפ",
    "טראנס",
    "האוס",
];
static HE_GENRE_LEN: usize = HE_GENRE.len();


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_item_is_in_list() {
// 		let result = genre();
// 		assert!(HE_GENRE.contains(&result.as_str()), "The animal should be in the ANIMALS list.");
//         print!("--------------");// This assumes you can iterate over EN_NAME, which would require it to be a Vec or a slice
//         println!("{}", result);
        
// 	}

// 	#[test]
// 	fn test_len() {
// 		let length = 20;
// 		assert!(HE_GENRE.len() == length, "The length of the list is not equal to the setting.");
// 	}

	 
// }
