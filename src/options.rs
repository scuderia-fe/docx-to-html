pub struct Options {
  pub style_map: Vec<(String, String)>,
}

impl Default for Options {
  fn default() -> Self {
    Options { style_map: vec![] }
  }
}
