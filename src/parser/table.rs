use docx_rs::{Table, TableCellContent, TableChild, TableRow, TableRowChild};

use crate::{
  element::{Element, ElementChildren, ElementTag},
  state::STYLE_MAP,
};

use super::{paragraph::analyze_paragraph, style::analyze_style};

fn analyze_row(row: &TableRow) -> Vec<ElementChildren> {
  let mut children = vec![];

  row.cells.iter().for_each(|child| {
    let TableRowChild::TableCell(cell) = child;
    let mut element = Element {
      tag: ElementTag::Td,
      styles: vec![
        "border-collapse: collapse; border-spacing: 0; border: 1px solid black;".to_owned(),
      ],
      ..Element::default()
    };

    cell.children.iter().for_each(|child| match child {
      TableCellContent::Paragraph(paragraph) => {
        let children = analyze_paragraph(paragraph);
        element.children.append(&mut vec![children]);
      }
      TableCellContent::Table(table) => {
        let mut children = vec![analyze_table(table)];
        element.children.append(&mut children);
      }
      _ => (),
    });

    children.push(ElementChildren::Element(element));
  });

  children
}

pub fn analyze_table(table: &Table) -> ElementChildren {
  let mut element = Element {
    tag: ElementTag::Table,
    styles: vec![
      "border-collapse: collapse; border-spacing: 0; border: 1px solid black;".to_owned(),
    ],
    ..Element::default()
  };

  if let Some(style) = table.property.get_style() {
    unsafe {
      if let Some(style) = STYLE_MAP.get(&style.val) {
        if let Some(based_on) = style.based_on.as_ref() {
          if let Some(based_on) = STYLE_MAP.get(&based_on.val) {
            element.styles.append(&mut analyze_style(based_on));
          }
        }

        element.styles.append(&mut analyze_style(style));
      }
    }
  };

  table.rows.iter().for_each(|child| {
    let TableChild::TableRow(row) = child;
    let mut row_element = Element {
      tag: ElementTag::Tr,
      styles: vec![
        "border-collapse: collapse; border-spacing: 0; border: 1px solid black;".to_owned(),
      ],
      ..Element::default()
    };

    let mut children = analyze_row(row);

    row_element.children.append(&mut children);
    element
      .children
      .append(&mut vec![ElementChildren::Element(row_element)]);
  });

  ElementChildren::Element(element)
}
