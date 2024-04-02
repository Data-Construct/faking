use wasm_bindgen::prelude::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::{locales::en::misc::colors, utils::seeder};

const FORMAT_BASE: &str = "data:image/svg+xml;";

// #[wasm_bindgen]
pub fn image_uri(props: Option<URI_Properties>) -> String {
  let mut height = seeder::gen_range(1..4000);
  let mut width = seeder::gen_range(1..4000);
  let mut color = colors::hex_color();
  let mut svg_type = ENCODE_FORMAT[seeder::gen_range(0..=1)];

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

    if p.color.is_some() {
      color = p.color.unwrap().to_string();
    }

    if p.svg_type.is_some() {
      svg_type = p.svg_type.unwrap();
    }
  }

  let half_height = height / 2;
  let half_width = width / 2;

  let tag_svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" baseProfile="full" width="{}" height="{}">"#, width, height);
  let tag_rect = format!(r#"<rect width="100%" height="100%" fill="{}"/>"#, color);
  let tag_text_outer = format!(r#"<text x="{}" y="{}" font-size="20" alignment-baseline="middle" text-anchor="middle" fill="white">"#, half_width, half_height);
  let tag_text_inner = format!("{}x{}", width, height);

  let svg = concat_string!(tag_svg, tag_rect, tag_text_outer, tag_text_inner, "</text>", "</svg>");
  if svg_type == "svg-uri" {
    return to_svg_encode(svg);
  }
  return to_base64_encode(svg);
}

fn to_base64_encode(svg: String) -> String {
  let encoded_text = STANDARD.encode(svg);
  format!("{}base64,{}", FORMAT_BASE, encoded_text)
}

fn to_svg_encode(svg: String) -> String {
  let encoded_text: String = url::form_urlencoded::byte_serialize(svg.as_bytes()).collect();
  format!("{}charset=UTF-8,{}", FORMAT_BASE, encoded_text)
}

// #[wasm_bindgen]
#[derive(Clone)]
pub struct URI_Properties {
  pub height: Option<u16>,
  pub width: Option<u16>,
  pub color: Option<&'static str>,
  pub svg_type: Option<&'static str>,
}

static ENCODE_FORMAT: [&'static str; 2] = [
  "svg-uri",
  "svg-base64"
];

// #[cfg(test)]
// mod tests {
//     use crate::{image::uri::image_uri, utils::seeder};

// 	#[test]
// 	fn it_works() {
// 		seeder::set_seed(1);
//     println!("{}", image_uri(None));
// 	}
// }