use docx_rs::{Drawing, DrawingData, Run, RunChild, RunProperty, Text, VertAlignType};

use crate::{
  element::{ElementChildren, ElementTag},
  image::HtmlImage,
  state::IMAGES,
};

pub struct RunElement {
  pub tags: Vec<ElementTag>,
  pub text: String,
  pub style: Vec<String>,
}

impl RunElement {
  pub fn to_string(&self) -> String {
    let mut string = String::new();

    self.tags.iter().enumerate().for_each(|(index, tag)| {
      if index == 0 {
        string.push_str(&format!(
          "<{} style=\"{}\">",
          tag.to_string(),
          self.style.join(";")
        ));
      } else {
        string.push_str(&format!("<{}>", tag.to_string()));
      }
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
    style: vec![],
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

  if let Some(color) = &run_properties.color.as_ref() {
    let value = &color.val;
    if value.len().eq(&6) || value.len().eq(&8) {
      if let Ok(_) = u32::from_str_radix(&value, 16) {
        element.style.push(format!("color: #{}", value));
      } else {
        element.style.push(format!("color: {}", value));
      }
    } else {
      element.style.push(format!("color: {}", value));
    }
  }

  if let Some(vert_align) = &run_properties.vert_align {
    match vert_align.val {
      VertAlignType::SuperScript => element.tags.push(ElementTag::Sup),
      VertAlignType::SubScript => element.tags.push(ElementTag::Sub),
      VertAlignType::Baseline => (),
      _ => (),
    }
  };

  element
}

fn analyze_run_text(text: &Text) -> Option<ElementChildren> {
  if text.text.is_empty() {
    return None;
  }

  Some(ElementChildren::Text(text.text.to_string()))
}

fn analyze_run_image(image: &Drawing) -> Option<ElementChildren> {
  match &image.data {
    Some(DrawingData::Pic(pic)) => unsafe {
      let image = IMAGES.iter().find(|picture| picture.id.eq(&pic.id));
      if image.is_none() {
        return None;
      }

      let image = image.unwrap();
      if image.src.is_empty() {
        return None;
      }

      let img = HtmlImage {
        id: image.id.clone(),
        src: image.src.clone(),
        size: pic.size,
      };

      return Some(ElementChildren::Text(img.to_string()));
    },
    Some(DrawingData::TextBox(_)) => None,
    None => None,
  }
}

pub fn analyze_run(run: &Run) -> Vec<ElementChildren> {
  let mut element = analyze_run_properties(&run.run_property);

  let children = run
    .children
    .iter()
    .filter_map(|child| match child {
      RunChild::Text(text) => analyze_run_text(text),
      RunChild::Drawing(image) => analyze_run_image(image),
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
