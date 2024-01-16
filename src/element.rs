#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};

pub static mut CONTAINER: Element = Element {
  tag: ElementTag::Div,
  children: vec![],
  styles: vec![],
  classes: vec![],
};

pub enum ElementTag {
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Span,
  Div,
  P,
  Em,
  Strong,
  A,
  Img,
  Ul,
  Ol,
  Li,
  Table,
  Tr,
  Td,
  Th,
  Thead,
  Tbody,
  Tfoot,
  Blockquote,
  Pre,
  Code,
  Hr,
  Br,
  Sub,
  Sup,
}

impl ElementTag {
  pub fn from_style(style: &str) -> Self {
    match style {
      "Title" => ElementTag::H1,
      "Heading1" => ElementTag::H2,
      "Heading2" => ElementTag::H3,
      "Heading3" => ElementTag::H4,
      "Heading4" => ElementTag::H5,
      "Heading5" => ElementTag::H6,
      "FootnoteText" => ElementTag::Sub,
      "SubtleEmphasis" => ElementTag::Em,
      "IntenseEmphasis" => ElementTag::Strong,
      "FootnoteReference" => ElementTag::Sup,
      "Hyperlink" => ElementTag::A,
      "Strong" => ElementTag::Strong,
      _ => ElementTag::P,
    }
  }

  fn to_value(&self) -> &str {
    match self {
      ElementTag::H1 => "h1",
      ElementTag::H2 => "h2",
      ElementTag::H3 => "h3",
      ElementTag::H4 => "h4",
      ElementTag::H5 => "h5",
      ElementTag::H6 => "h6",
      ElementTag::Span => "span",
      ElementTag::Div => "div",
      ElementTag::P => "p",
      ElementTag::Em => "em",
      ElementTag::Strong => "strong",
      ElementTag::A => "a",
      ElementTag::Img => "img",
      ElementTag::Ul => "ul",
      ElementTag::Ol => "ol",
      ElementTag::Li => "li",
      ElementTag::Table => "table",
      ElementTag::Tr => "tr",
      ElementTag::Td => "td",
      ElementTag::Th => "th",
      ElementTag::Thead => "thead",
      ElementTag::Tbody => "tbody",
      ElementTag::Tfoot => "tfoot",
      ElementTag::Blockquote => "blockquote",
      ElementTag::Pre => "pre",
      ElementTag::Code => "code",
      ElementTag::Hr => "hr",
      ElementTag::Br => "br",
      ElementTag::Sub => "sub",
      ElementTag::Sup => "sup",
    }
  }
}

impl Display for ElementTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.to_value())
  }
}

pub enum ElementChildren {
  Text(String),
  Element(Element),
}

pub struct Element {
  pub tag: ElementTag,
  pub styles: Vec<String>,
  pub classes: Vec<String>,
  pub children: Vec<ElementChildren>,
}

impl Element {
  pub fn to_string(&self) -> String {
    let style = self.styles.join("; ");
    let classes = self.classes.join(" ");

    let mut element = format!("<{} style=\"{}\" classes=\"{}\">", self.tag, style, classes);

    self.children.iter().for_each(|child| match child {
      ElementChildren::Text(text) => element.push_str(text),
      ElementChildren::Element(child) => element.push_str(&child.to_string()),
    });

    element.push_str(&format!("</{}>", self.tag.to_value()));

    element
  }
}

impl Default for Element {
  fn default() -> Self {
    Element {
      tag: ElementTag::P,
      children: vec![],
      styles: vec![],
      classes: vec![],
    }
  }
}
