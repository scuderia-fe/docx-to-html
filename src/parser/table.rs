use docx_rs::{Table, TableCellContent, TableChild, TableRow, TableRowChild};

use crate::element::{Element, ElementChildren, ElementTag};

use super::paragraph::analyze_paragraph;

fn analyze_row(row: &TableRow) -> Vec<ElementChildren> {
  let mut children = vec![];

  row.cells.iter().for_each(|child| {
    let TableRowChild::TableCell(cell) = child;
    let mut element = Element {
      tag: ElementTag::Td,
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
    ..Element::default()
  };

  table.rows.iter().for_each(|child| {
    let TableChild::TableRow(row) = child;
    let mut row_element = Element {
      tag: ElementTag::Tr,
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
