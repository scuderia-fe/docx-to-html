#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq)]
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
  U,
  Ul,
  Ol,
  Li,
  Table,
  Tr,
  Td,
  Th,
  Thead,
  I,
  Tbody,
  Tfoot,
  Blockquote,
  Pre,
  Code,
  Hr,
  Br,
  Sub,
  Sup,
  S,
  Mark,
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
      "Bold" => ElementTag::Strong,
      "BoldCS" => ElementTag::Strong,
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
      ElementTag::I => "i",
      ElementTag::Blockquote => "blockquote",
      ElementTag::Pre => "pre",
      ElementTag::Code => "code",
      ElementTag::Hr => "hr",
      ElementTag::Br => "br",
      ElementTag::Sub => "sub",
      ElementTag::Sup => "sup",
      ElementTag::S => "s",
      ElementTag::U => "u",
      ElementTag::Mark => "mark",
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
  pub id: Option<String>,
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
      id: None,
      tag: ElementTag::P,
      children: vec![],
      styles: vec![],
      classes: vec![],
    }
  }
}
