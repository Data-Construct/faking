use wasm_bindgen::prelude::*;
use url::form_urlencoded;

use crate::{locales::en::misc::{colors, lorem_ipsum}, utils::seeder};

const URL_BASE: &str = "https://via.placeholder.com";

#[wasm_bindgen]
pub fn image_placeholder(props: Option<Placeholder_Properties>) -> String {
  let mut height = seeder::gen_range(1..4000);
  let mut width = seeder::gen_range(1..4000);
  let mut background_color = colors::hex_color();
  let mut text_color = colors::hex_color();
  let mut image_format = FILE_FORMATS[seeder::gen_range(0..5)].to_string();
  let mut text = lorem_ipsum::lorem_ipsum_word();

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
      };
    }

    if p.background_color.is_some() {
      background_color = p.background_color.unwrap().to_string();
    }

    if p.text_color.is_some() {
      text_color = p.text_color.unwrap().to_string();
    }

    if p.image_format.is_some() {
      image_format = p.image_format.unwrap().to_string();
    }

    if p.text.is_some() {
      text = p.text.unwrap().to_string();
    }
  }

  let encoded_text: String = url::form_urlencoded::byte_serialize(text.as_bytes()).collect();
  format!("{}/{}x{}/{}/{}.{}?text={}", URL_BASE, width.to_string(), height.to_string(), background_color, text_color, image_format, encoded_text)
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Placeholder_Properties {
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub background_color: Option<&'static str>,
    pub text_color: Option<&'static str>,
    pub image_format: Option<&'static str>,
    pub text: Option<&'static str>,
}

static FILE_FORMATS: [&'static str; 5] = [
  "gif",
  "jpeg",
  "jpg",
  "png",
  "webp"
];
