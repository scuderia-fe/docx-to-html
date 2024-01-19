use docx_rs::{Paragraph, ParagraphChild, ParagraphProperty};

use crate::{
  alert,
  element::{Element, ElementChildren, ElementTag},
};

use super::{
  hyperlink::analyze_hyperlink,
  run::{analyze_run, analyze_run_properties},
  style::analyze_style,
};

pub fn get_paragraph_properties(properties: &ParagraphProperty) -> Vec<String> {
  let mut props: Vec<String> = vec![];

  if let Some(alignment) = &properties.alignment.as_ref() {
    props.push(format!("text-align: {}", alignment.val));
  };

  if let Some(style) = properties.style.as_ref() {
    unsafe {
      if let Some(style) = crate::state::STYLE_MAP.get(&style.val) {
        if let Some(based_on) = style.based_on.as_ref() {
          if let Some(based_on) = crate::state::STYLE_MAP.get(&based_on.val) {
            props.append(&mut analyze_style(&based_on));
          }
        }

        props.append(&mut analyze_style(&style));
      }
    }
  }

  props
}

pub fn analyze_paragraph(paragraph: &Paragraph) -> ElementChildren {
  let mut element = Element::default();

  let tag = &paragraph.property.style.as_ref();
  if let Some(tag) = tag {
    if tag.val.eq("ListParagraph") {
      let numbering_property = &paragraph.property.numbering_property.as_ref();
      if let Some(property) = numbering_property {
        let id = &property.id.as_ref().unwrap().id;
        match id {
          _ => element.tag = ElementTag::Ul,
        }
      }
    } else {
      let tag = &tag.val;
      element.tag = ElementTag::from_style(tag);
    }
  }

  let mut run_property = analyze_run_properties(&paragraph.property.run_property);

  let properties = get_paragraph_properties(&paragraph.property);
  element.styles = properties;

  paragraph.children.iter().for_each(|child| match child {
    ParagraphChild::Run(run) => {
      let mut children = analyze_run(run);
      if element.tag == ElementTag::Ul || element.tag == ElementTag::Ol {
        children.iter_mut().for_each(|child| {
          if let ElementChildren::Element(child) = child {
            alert(&format!("child: {}", child.to_string()));
            child.tag = ElementTag::Li;
          }
        });
      }
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
