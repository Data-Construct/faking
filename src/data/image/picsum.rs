use rand::{distributions::Alphanumeric};
use wasm_bindgen::prelude::*;

use crate::utils::seeder;

const URL_BASE: &str = "https://picsum.photos/seed";

#[wasm_bindgen]
pub fn image_picsum(props: Option<Picsum_Properties>) -> String {
  let mut height: u16 = seeder::gen_range(1..4000);
  let mut width: u16 = seeder::gen_range(1..4000);
  let mut greyscale: bool = seeder::gen();
  let mut blur: u8 = seeder::gen_range(0..10);

  let seed_length = seeder::gen_range(5..=10);
  let seed: String = seeder::sample_iter(&Alphanumeric).take(seed_length).map(char::from).collect();

  if props.is_some() {
    let p = props.unwrap();
    if p.height.is_some() {
      let h = p.height.unwrap();
      height = if h < 1 {
        1
      } else if h >= 4000 {
        3999
      } else {
        h
      }
    }

    if p.width.is_some() {
      let w = p.width.unwrap();
      width = if w < 1 {
        1
      } else if w >= 4000 {
        3999
      } else {
        w
      }
    }

    if p.greyscale.is_some() {
      greyscale = p.greyscale.unwrap();
    }

    if p.blur.is_some() {
      let b = p.blur.unwrap();
      blur = if b > 10 {
        10
      } else {
        b
      };
    }
  }

  let is_valid_blur = 0 < blur && blur <= 10;

  let mut query = "".to_string();
  if greyscale || is_valid_blur {
    query.push_str("?");
    
    if greyscale {
      query.push_str("greyscale");

      if is_valid_blur {
        query.push_str("&");
      }
    }

    if is_valid_blur {
      query.push_str("blur=");
      query.push_str(&blur.to_string())
    }
  }

  format!("{}/{}/{}/{}{}", URL_BASE, seed, width.to_string(), height.to_string(), query)
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Picsum_Properties {
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub greyscale: Option<bool>,
    pub blur: Option<u8>,
}

