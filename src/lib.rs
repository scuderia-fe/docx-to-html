mod element;
mod image;
mod options;
mod parser;
mod state;
mod utils;

use base64::{engine::general_purpose, Engine as _};
use docx_rs::read_docx;
use image::HtmlImage;
use state::{CONTAINER, IMAGES};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn convert(file: &[u8]) -> String {
  utils::set_panic_hook();
  unsafe {
    CONTAINER.children.clear();
    IMAGES.clear()
  }

  let mut document = read_docx(file).unwrap();

  let images = &document.images;
  images.iter().for_each(|img| {
    let (id, _file_type, image, _png) = img;

    let src = general_purpose::STANDARD.encode(&image.0);
    let image = HtmlImage {
      id: id.to_string(),
      src: format!("data:image/png;base64,{}", src),
      ..Default::default()
    };

    unsafe { IMAGES.push(image) }
  });

  document.styles.styles.iter_mut().for_each(|style| {
    let style = style.clone();
    let id = style.style_id.clone();
    unsafe { state::STYLE_MAP.insert(id, style) };
  });

  document
    .document
    .children
    .iter()
    .for_each(parser::parse_child);

  unsafe { CONTAINER.to_string() }
}
