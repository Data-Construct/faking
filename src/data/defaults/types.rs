use crate::utils::seeder;
use wasm_bindgen::prelude::*;

// bool

#[wasm_bindgen]
pub fn bool() -> bool {
	seeder::gen::<bool>()
}

// signed int

#[wasm_bindgen]
pub fn i8() -> i8 {
	seeder::gen::<i8>()
}

#[wasm_bindgen]
pub fn i16() -> i16 {
	seeder::gen::<i16>()
}

#[wasm_bindgen]
pub fn i32() -> i32 {
	seeder::gen::<i32>()
}

#[wasm_bindgen]
pub fn i64() -> i64 {
	seeder::gen::<i64>()
}

#[wasm_bindgen]
pub fn isize() -> isize {
	seeder::gen::<isize>()
}

// unsinged int

#[wasm_bindgen]
pub fn u8() -> u8 {
	seeder::gen::<u8>()
}

#[wasm_bindgen]
pub fn u16() -> u16 {
	seeder::gen::<u16>()
}

#[wasm_bindgen]
pub fn u32() -> u32 {
	seeder::gen::<u32>()
}

#[wasm_bindgen]
pub fn u64() -> u64 {
	seeder::gen::<u64>()
}

#[wasm_bindgen]
pub fn usize() -> usize {
	seeder::gen::<usize>()
}

// floats

#[wasm_bindgen]
pub fn f32() -> f32 {
	seeder::gen::<f32>()
}

#[wasm_bindgen]
pub fn f64() -> f64 {
	seeder::gen::<f64>()
}
