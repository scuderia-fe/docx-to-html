use docx_rs::{RunProperty, Style};

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

  if run_properties.sz.is_some() {
    accumulator.push(format!(
      "font-size: {}px",
      run_properties.sz.as_ref().unwrap().val
    ));
  };

  if run_properties.strike.is_some() {
    accumulator.push("text-decoration: line-through".to_owned());
  };

  if run_properties.vanish.is_some() {
    accumulator.push("visibility: hidden".to_owned());
  };

  accumulator
}

pub fn analyze_style(style: &Style) -> Vec<String> {
  let mut accumulator: Vec<String> = vec![];

  accumulator.append(&mut analyze_run_properties(&style.run_property));

  return accumulator;
}
