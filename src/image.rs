pub struct HtmlImage {
  pub id: String,
  pub src: String,
  pub size: (u32, u32),
}

impl HtmlImage {
  pub fn to_string(&self) -> String {
    format!(
      "<img id=\"{}\" src=\"{}\" width=\"{}\" height=\"{}\" />",
      self.id,
      self.src,
      format!("{}px", self.size.0 / 10000),
      format!("{}px", self.size.1 / 10000),
    )
  }
}

impl Default for HtmlImage {
  fn default() -> Self {
    HtmlImage {
      id: String::new(),
      src: String::new(),
      size: (0, 0),
    }
  }
}
