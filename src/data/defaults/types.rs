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
pub fn i8_range(min: i8, max: i8) -> i8 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn i16() -> i16 {
	seeder::gen::<i16>()
}

#[wasm_bindgen]
pub fn i16_range(min: i16, max: i16) -> i16 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn i32() -> i32 {
	seeder::gen::<i32>()
}

#[wasm_bindgen]
pub fn i32_range(min: i32, max: i32) -> i32 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn i64() -> i64 {
	seeder::gen::<i64>()
}

#[wasm_bindgen]
pub fn i64_range(min: i64, max: i64) -> i64 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn isize() -> isize {
	seeder::gen::<isize>()
}

#[wasm_bindgen]
pub fn isize_range(min: isize, max: isize) -> isize {
	seeder::gen_range(min..max)
}

// unsinged int

#[wasm_bindgen]
pub fn u8() -> u8 {
	seeder::gen::<u8>()
}

#[wasm_bindgen]
pub fn u8_range(min: u8, max: u8) -> u8 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn u16() -> u16 {
	seeder::gen::<u16>()
}

#[wasm_bindgen]
pub fn u16_range(min: u16, max: u16) -> u16 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn u32() -> u32 {
	seeder::gen::<u32>()
}

#[wasm_bindgen]
pub fn u32_range(min: u32, max: u32) -> u32 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn u64() -> u64 {
	seeder::gen::<u64>()
}

#[wasm_bindgen]
pub fn u64_range(min: u64, max: u64) -> u64 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn usize() -> usize {
	seeder::gen::<usize>()
}

#[wasm_bindgen]
pub fn usize_range(min: usize, max: usize) -> usize {
	seeder::gen_range(min..max)
}

// floats

#[wasm_bindgen]
pub fn f32() -> f32 {
	seeder::gen::<f32>()
}

#[wasm_bindgen]
pub fn f32_range(min: f32, max: f32) -> f32 {
	seeder::gen_range(min..max)
}

#[wasm_bindgen]
pub fn f64() -> f64 {
	seeder::gen::<f64>()
}

#[wasm_bindgen]
pub fn f64_range(min: f64, max: f64) -> f64 {
	seeder::gen_range(min..max)
}
