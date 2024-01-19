use docx_rs::{ParagraphProperty, RunProperty, Style};

fn analyze_run_properties(run_properties: &RunProperty) -> Vec<String> {
  let mut accumulator: Vec<String> = vec![];

  if run_properties.bold.is_some() {
    accumulator.push("font-weight: bold".to_owned());
  };

  if run_properties.italic.is_some() {
    accumulator.push("font-style: italic".to_owned());
  };

  if run_properties.underline.is_some() {
    accumulator.push("text-decoration: underline".to_owned());
  };

  // if run_properties.sz.is_some() {
  //   accumulator.push(format!(
  //     "font-size: {}px",
  //     run_properties.sz.as_ref().unwrap().val
  //   ));
  // };

  if run_properties.strike.is_some() {
    accumulator.push("text-decoration: line-through".to_owned());
  };

  if run_properties.vanish.is_some() {
    accumulator.push("visibility: hidden".to_owned());
  };

  if let Some(color) = &run_properties.color.as_ref() {
    let value = &color.val;
    if value.len().eq(&6) || value.len().eq(&8) {
      if let Ok(_) = u32::from_str_radix(&value, 16) {
        accumulator.push(format!("color: #{}", value));
      } else {
        accumulator.push(format!("color: {}", value));
      }
    } else {
      accumulator.push(format!("color: {}", value));
    }
  }

  accumulator
}

pub fn analyze_paragraph_properties(properties: &ParagraphProperty) -> Vec<String> {
  let mut accumulator: Vec<String> = vec![];

  if let Some(alignment) = &properties.alignment.as_ref() {
    accumulator.push(format!("text-align: {}", alignment.val));
  };

  accumulator
}

pub fn analyze_style(style: &Style) -> Vec<String> {
  let mut accumulator: Vec<String> = vec![];

  accumulator.append(&mut analyze_run_properties(&style.run_property));
  accumulator.append(&mut analyze_paragraph_properties(&style.paragraph_property));

  return accumulator;
}
