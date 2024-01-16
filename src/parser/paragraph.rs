use docx_rs::{Paragraph, ParagraphChild};

use crate::element::{Element, ElementChildren, ElementTag};

use super::{hyperlink::analyze_hyperlink, run::analyze_run};

pub fn analyze_paragraph(paragraph: &Paragraph) -> ElementChildren {
  let mut element = Element::default();

  let styles = &paragraph.property.style.as_ref();
  if styles.is_some() {
    let styles = &styles.unwrap().val;
    element.tag = ElementTag::from_style(styles)
  }

  paragraph.children.iter().for_each(|child| match child {
    ParagraphChild::Run(run) => {
      let mut children = analyze_run(run);
      element.children.append(&mut children);
    }
    ParagraphChild::Hyperlink(hyperlink) => {
      let children = analyze_hyperlink(hyperlink);
      element
        .children
        .append(&mut vec![ElementChildren::Element(children)]);
    }
    _ => (),
  });

  ElementChildren::Element(element)
}
