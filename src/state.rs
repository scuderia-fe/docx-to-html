use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use docx_rs::Style;

use crate::{
  element::{Element, ElementTag},
  image::HtmlImage,
  options::Options,
};

pub static mut CONTAINER: Element = Element {
  id: None,
  tag: ElementTag::Div,
  children: vec![],
  styles: vec![],
  classes: vec![],
};

#[allow(dead_code)]
pub static mut OPTIONS: Options = Options { style_map: vec![] };

pub static mut IMAGES: Vec<HtmlImage> = Vec::new();

pub static mut STYLE_MAP: Lazy<HashMap<String, Style>> = Lazy::new(|| {
  let map: HashMap<String, Style> = HashMap::new();
  return map;
});
