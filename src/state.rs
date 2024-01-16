use crate::{
  element::{Element, ElementTag},
  options::Options,
};

pub static mut CONTAINER: Element = Element {
  tag: ElementTag::Div,
  children: vec![],
  styles: vec![],
  classes: vec![],
};

#[allow(dead_code)]
pub static mut OPTIONS: Options = Options { style_map: vec![] };
