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
