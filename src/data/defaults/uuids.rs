use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuid_v1() -> String {
	let context = uuid::Context::new(crate::data::defaults::types::u16());
	let ts = uuid::Timestamp::from_unix(&context, 1497624119, 1234);
	Uuid::new_v1(
		ts,
		&[
			1, 2, 3, 4, 5, 6,
		],
	)
	.to_string()
}

#[wasm_bindgen]
pub fn uuid_v3() -> String {
	Uuid::new_v3(
		&Uuid::NAMESPACE_DNS,
		crate::data::defaults::name::last_name().as_bytes(),
	)
	.to_string()
}

#[wasm_bindgen]
pub fn uuid_v4() -> String {
	Uuid::new_v4().to_string()
}

#[wasm_bindgen]
pub fn uuid_v5() -> String {
	Uuid::new_v5(
		&Uuid::NAMESPACE_DNS,
		crate::data::defaults::name::last_name().as_bytes(),
	)
	.to_string()
}

// TODO(clearfeld): v6 to 8 not yet stable in uuid crate

// #[wasm_bindgen]
// pub fn uuid_v6() -> String {
//     Uuid::new_v6().to_string()
// }

// #[wasm_bindgen]
// pub fn uuid_v7() -> String {
//     Uuid::new_v7().to_string()
// }

// #[wasm_bindgen]
// pub fn uuid_v8() -> String {
//     Uuid::new_v8().to_string()
//}
