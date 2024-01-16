use docx_rs::{Run, RunChild, Text};

use crate::element::{Element, ElementChildren, ElementTag};

fn analyze_run_text(text: &Text) -> Option<ElementChildren> {
  if text.text.is_empty() {
    return None;
  }

  Some(ElementChildren::Text(text.text.to_string()))
}

pub fn analyze_run(run: &Run) -> Vec<ElementChildren> {
  let children = run
    .children
    .iter()
    .filter_map(|child| match child {
      RunChild::Text(text) => analyze_run_text(text),
      _ => None,
    })
    .collect();

  let style = &run.run_property.style.clone().unwrap_or_default();
  if !style.val.is_empty() && style.val != "Normal" {
    let element = Element {
      tag: ElementTag::from_style(&style.val),
      ..Element::default()
    };

    vec![ElementChildren::Element(element)]
  } else {
    children
  }
}
