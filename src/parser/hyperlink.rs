use docx_rs::{Hyperlink, ParagraphChild};

use crate::element::{Element, ElementTag};

use super::run::analyze_run;

pub fn analyze_hyperlink(hyperlink: &Hyperlink) -> Element {
  let mut element = Element {
    tag: ElementTag::A,
    ..Element::default()
  };

  hyperlink.children.iter().for_each(|child| match child {
    ParagraphChild::Run(run) => {
      let mut children = analyze_run(run);
      element.children.append(&mut children);
    }
    _ => (),
  });

  element
}
