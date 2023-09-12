use rand::Rng;
use wasm_bindgen::prelude::*;

// bool

#[wasm_bindgen]
pub fn bool() -> bool {
	let mut rng = rand::thread_rng();
	rng.gen::<bool>()
}

// signed int

#[wasm_bindgen]
pub fn i8() -> i8 {
	let mut rng = rand::thread_rng();
	rng.gen::<i8>()
}

#[wasm_bindgen]
pub fn i16() -> i16 {
	let mut rng = rand::thread_rng();
	rng.gen::<i16>()
}

#[wasm_bindgen]
pub fn i32() -> i32 {
	let mut rng = rand::thread_rng();
	rng.gen::<i32>()
}

#[wasm_bindgen]
pub fn i64() -> i64 {
	let mut rng = rand::thread_rng();
	rng.gen::<i64>()
}

#[wasm_bindgen]
pub fn isize() -> isize {
	let mut rng = rand::thread_rng();
	rng.gen::<isize>()
}

// unsinged int

#[wasm_bindgen]
pub fn u8() -> u8 {
	let mut rng = rand::thread_rng();
	rng.gen::<u8>()
}

#[wasm_bindgen]
pub fn u16() -> u16 {
	let mut rng = rand::thread_rng();
	rng.gen::<u16>()
}

#[wasm_bindgen]
pub fn u32() -> u32 {
	let mut rng = rand::thread_rng();
	rng.gen::<u32>()
}

#[wasm_bindgen]
pub fn u64() -> u64 {
	let mut rng = rand::thread_rng();
	rng.gen::<u64>()
}

#[wasm_bindgen]
pub fn usize() -> usize {
	let mut rng = rand::thread_rng();
	rng.gen::<usize>()
}

// floats

#[wasm_bindgen]
pub fn f32() -> f32 {
	let mut rng = rand::thread_rng();
	rng.gen::<f32>()
}

#[wasm_bindgen]
pub fn f64() -> f64 {
	let mut rng = rand::thread_rng();
	rng.gen::<f64>()
}
