use wasm_bindgen::prelude::*;
use bitcoin::{Address, PublicKey, Network};
use bitcoin::secp256k1::{rand, Secp256k1};

#[wasm_bindgen]
pub fn mainnet_address() -> String {
    let public_key = generate_key();
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    address.to_string()
}

#[wasm_bindgen]
pub fn testnet_address() -> String {
    let public_key = generate_key();
    let address = Address::p2pkh(&public_key, Network::Testnet);
    address.to_string()
}

#[wasm_bindgen]
pub fn signet_address() -> String {
    let public_key = generate_key();
    let address = Address::p2pkh(&public_key, Network::Signet);
    address.to_string()
}

#[wasm_bindgen]
pub fn regtest_address() -> String {
    let public_key = generate_key();
    let address = Address::p2pkh(&public_key, Network::Regtest);
    address.to_string()
}

pub fn generate_key() -> PublicKey {
    let s = Secp256k1::new();
    PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1)
}