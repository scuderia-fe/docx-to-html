mod element;
mod options;
mod parser;
mod state;
mod utils;

use docx_rs::read_docx;
use state::CONTAINER;
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
  }

  let document = read_docx(file).unwrap();
  // let images = &document.images;
  // alert(format!("images: {:?}", images).as_str());

  document
    .document
    .children
    .iter()
    .for_each(parser::parse_child);

  unsafe { CONTAINER.to_string() }
}
