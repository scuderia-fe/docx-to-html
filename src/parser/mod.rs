mod hyperlink;
mod paragraph;
mod run;
mod table;

use docx_rs::DocumentChild;

use crate::element::CONTAINER;

pub fn parse_child(child: &DocumentChild) {
  let children = match child {
    DocumentChild::Paragraph(paragraph) => Some(paragraph::analyze_paragraph(paragraph)),
    DocumentChild::Table(table) => Some(table::analyze_table(table)),
    _ => None,
  };

  if children.is_none() {
    return;
  }
  let children = children.unwrap();

  unsafe { CONTAINER.children.append(&mut vec![children]) }
}
