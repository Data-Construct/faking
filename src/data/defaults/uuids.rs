use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uuid_v1() -> String {
	Uuid::new_v1(
		uuid::Timestamp::from_unix(
			&uuid::Context::new(crate::data::defaults::types::u16()),
			1497624119,
			1234,
		),
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

#[wasm_bindgen]
pub fn uuid_v6() -> String {
	Uuid::now_v6(&[
		1, 2, 3, 4, 5, 6,
	])
	.to_string()
}

#[wasm_bindgen]
pub fn uuid_v7() -> String {
	Uuid::now_v7().to_string()
}

#[wasm_bindgen]
pub fn uuid_v8() -> String {
	Uuid::new_v8(IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(6))).to_string()
}
