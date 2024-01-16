use docx_rs::{Run, RunChild, RunProperty, Text};

use crate::element::{ElementChildren, ElementTag};

pub struct RunElement {
  pub tags: Vec<ElementTag>,
  pub text: String,
}

impl RunElement {
  pub fn to_string(&self) -> String {
    let mut string = String::new();

    self.tags.iter().for_each(|tag| {
      string.push_str(&format!("<{}>", tag.to_string()));
    });

    string.push_str(&self.text);

    self.tags.iter().rev().for_each(|tag| {
      string.push_str(&format!("</{}>", tag.to_string()));
    });

    string
  }
}

pub fn analyze_run_properties(run_properties: &RunProperty) -> RunElement {
  let mut element = RunElement {
    tags: vec![ElementTag::Span],
    text: String::new(),
  };

  if let Some(style) = &run_properties.style {
    element.tags.push(ElementTag::from_style(&style.val));
  }

  if run_properties.bold.is_some() {
    element.tags.push(ElementTag::Strong);
  };

  if run_properties.italic.is_some() {
    element.tags.push(ElementTag::I);
  };

  if run_properties.underline.is_some() {
    element.tags.push(ElementTag::U);
  };

  if run_properties.strike.is_some() {
    element.tags.push(ElementTag::S);
  };

  if run_properties.highlight.is_some() {
    element.tags.push(ElementTag::Mark);
  };

  // TODO: superscript and subscript
  // if run.run_property.vert_align.is_some() {
  //   if let Some(val) = &run.run_property.vert_align.as_ref().unwrap().val {
  //     match val.as_str() {
  //       "superscript" => element.tags.push(ElementTag::Sup),
  //       "subscript" => element.tags.push(ElementTag::Sub),
  //       _ => (),
  //     }
  //   }
  // };

  element
}

fn analyze_run_text(text: &Text) -> Option<ElementChildren> {
  if text.text.is_empty() {
    return None;
  }

  Some(ElementChildren::Text(text.text.to_string()))
}

pub fn analyze_run(run: &Run) -> Vec<ElementChildren> {
  let mut element = analyze_run_properties(&run.run_property);

  let children = run
    .children
    .iter()
    .filter_map(|child| match child {
      RunChild::Text(text) => analyze_run_text(text),
      _ => None,
    })
    .collect();

  if element.tags.len() == 0 {
    return children;
  }

  element.text = children
    .iter()
    .map(|child| match child {
      ElementChildren::Text(text) => text,
      _ => "",
    })
    .collect();

  vec![ElementChildren::Text(element.to_string())]
}
