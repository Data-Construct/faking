use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

// TODO: this needs to support seed repro

/// as per [new_v1](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v1)
pub fn uuid_v1() -> Uuid {
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
}

/// as per [new_v3](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v3)
pub fn uuid_v3() -> Uuid {
	Uuid::new_v3(
		&Uuid::NAMESPACE_DNS,
		crate::data::defaults::name::last_name().as_bytes(),
	)
}

/// as per [new_v4](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v4)
pub fn uuid_v4() -> Uuid {
	Uuid::new_v4()
}

/// as per [new_v5](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v5)
pub fn uuid_v5() -> Uuid {
	Uuid::new_v5(
		&Uuid::NAMESPACE_DNS,
		crate::data::defaults::name::last_name().as_bytes(),
	)
}

/// as per [now_v6](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.now_v6)
pub fn uuid_v6() -> Uuid {
	Uuid::now_v6(&[
		1, 2, 3, 4, 5, 6,
	])
}

/// as per [now_v7](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.now_v7)
pub fn uuid_v7() -> Uuid {
	Uuid::now_v7()
}

/// as per [new_v8](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v8)
pub fn uuid_v8() -> Uuid {
	Uuid::new_v8(IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(6)))
}

//

#[wasm_bindgen(js_name = uuid_v1)]
/// Returns a UUID v1 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v1();
/// ```
pub fn uuid_v1_wasm() -> String {
	uuid_v1().to_string()
}

#[wasm_bindgen(js_name = uuid_v3)]
/// Returns a UUID v3 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v3();
/// ```
pub fn uuid_v3_wasm() -> String {
	uuid_v3().to_string()
}

#[wasm_bindgen(js_name = uuid_v4)]
/// Returns a UUID v4 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v4();
/// ```
pub fn uuid_v4_wasm() -> String {
	uuid_v4().to_string()
}

#[wasm_bindgen(js_name = uuid_v5)]
/// Returns a UUID v5 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v5();
/// ```
pub fn uuid_v5_wasm() -> String {
	uuid_v5().to_string()
}

#[wasm_bindgen(js_name = uuid_v6)]
/// Returns a UUID v6 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v6();
/// ```
pub fn uuid_v6_wasm() -> String {
	uuid_v6().to_string()
}

#[wasm_bindgen(js_name = uuid_v7)]
/// Returns a UUID v7 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v7();
/// ```
pub fn uuid_v7_wasm() -> String {
	uuid_v7().to_string()
}

#[wasm_bindgen(js_name = uuid_v8)]
/// Returns a UUID v8 ([Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)).
///
/// @return
/// string
///
/// @example
/// ```ignore
/// faker.uuid_v8();
/// ```
pub fn uuid_v8_wasm() -> String {
	uuid_v8().to_string()
}
