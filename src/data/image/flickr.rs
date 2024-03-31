use wasm_bindgen::prelude::*;

use crate::utils::seeder;

const URL_BASE: &str = "https://loremflickr.com";

// #[wasm_bindgen]
pub fn image_flickr(props: Option<Flickr_Properties>) -> String {
  let mut height = seeder::gen_range(1..4000);
  let mut width = seeder::gen_range(1..4000);
  let mut category: String = "".to_string();
  let lock: u32 = seeder::gen();

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
      };
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

    if p.category.is_some() {
      let c = p.category.unwrap();
      category = c.to_string();
    }
  }

  format!("{}/{}/{}/{}?lock={}", URL_BASE, width.to_string(), height.to_string(), category, lock.to_string())
}

// #[wasm_bindgen]
#[derive(Clone)]
pub struct Flickr_Properties {
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub category: Option<&'static str>,
}
