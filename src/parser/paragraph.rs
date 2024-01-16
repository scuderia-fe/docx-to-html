use docx_rs::{Paragraph, ParagraphChild, ParagraphProperty};

use crate::element::{Element, ElementChildren, ElementTag};

use super::{
  hyperlink::analyze_hyperlink,
  run::{analyze_run, analyze_run_properties},
};

pub fn get_paragraph_properties(properties: &ParagraphProperty) -> Vec<String> {
  let mut props: Vec<String> = vec![];

  if let Some(alignment) = &properties.alignment.as_ref() {
    props.push(format!("text-align: {}", alignment.val));
  };

  props
}

pub fn analyze_paragraph(paragraph: &Paragraph) -> ElementChildren {
  let mut element = Element::default();

  let tag = &paragraph.property.style.as_ref();
  if tag.is_some() {
    let tag = &tag.unwrap().val;
    element.tag = ElementTag::from_style(tag)
  }

  let mut run_property = analyze_run_properties(&paragraph.property.run_property);

  let properties = get_paragraph_properties(&paragraph.property);
  element.styles = properties;

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

  if run_property.tags.len().eq(&0) {
    return ElementChildren::Element(element);
  }

  run_property.text = element.to_string();
  return ElementChildren::Text(run_property.to_string());
}
