#![doc = include_str!("../README.md")]

use {
    anyhow::Result,
    gstring::*,
    pulldown_cmark::{self as pd, Alignment},
    std::fmt::Write,
};

/// Process markdown content
pub fn process(input: &str) -> Result<String> {
    let mut s = String::new();

    // State
    let mut table = vec![];
    let mut align = None;
    let mut widths = vec![];
    let mut column = 0;
    let mut depth = 0;

    // Parse input into events
    for (event, range) in pd::Parser::new_ext(input, pd::Options::all()).into_offset_iter() {
        let source = &input[range.clone()];

        match event {
            pd::Event::Start(pd::Tag::Table(a)) => {
                // Start a new table
                table = vec![];
                align = Some(a);
                widths = vec![];
                column = 0;
                depth += 1;
            }
            pd::Event::Start(pd::Tag::TableHead) | pd::Event::Start(pd::Tag::TableRow) => {
                // Start a new table row
                table.push(vec![]);
            }
            pd::Event::Start(pd::Tag::TableCell) => {
                // Start a new table cell
                let row = table.last_mut().unwrap();
                row.push(GString::new());
                column += 1;
                if widths.len() < column {
                    widths.push(2);
                }
            }
            pd::Event::Start(_) => {
                depth += 1;
            }
            pd::Event::End(pd::TagEnd::TableCell) => {
                let row = table.last_mut().unwrap();
                let cell = row.last_mut().unwrap();
                let source = source.trim();
                cell.push(source);
                let i = column - 1;
                widths[i] = widths[i].max(source.len());
            }
            pd::Event::End(pd::TagEnd::TableHead) | pd::Event::End(pd::TagEnd::TableRow) => {
                // Reset the column index at end of table row
                column = 0;
            }
            pd::Event::End(pd::TagEnd::Table) => {
                // Decrement the level index
                depth -= 1;

                // Print the table
                for (r, row) in table.iter().enumerate() {
                    // Print each row
                    for (col, cell) in row.iter().enumerate() {
                        // Print each cell
                        let a = if col == 0 { "|" } else { "" };
                        match align.as_ref().unwrap()[col] {
                            Alignment::None => {
                                write!(&mut s, "{a} {:1$} |", cell.to_string(), widths[col])?;
                            }
                            Alignment::Left => {
                                write!(&mut s, "{a} {:<1$} |", cell.to_string(), widths[col])?;
                            }
                            Alignment::Right => {
                                write!(&mut s, "{a} {:>1$} |", cell.to_string(), widths[col])?;
                            }
                            Alignment::Center => {
                                write!(&mut s, "{a} {:^1$} |", cell.to_string(), widths[col])?;
                            }
                        }
                    }

                    if r == 0 {
                        // Print header line
                        writeln!(
                            &mut s,
                            "\n|{}|",
                            widths
                                .iter()
                                .enumerate()
                                .map(|(col, width)| {
                                    let (a, b) = match align.as_ref().unwrap()[col] {
                                        Alignment::None => ('-', '-'),
                                        Alignment::Left => (':', '-'),
                                        Alignment::Right => ('-', ':'),
                                        Alignment::Center => (':', ':'),
                                    };
                                    format!("{a}{}{b}", "-".repeat(*width))
                                })
                                .collect::<Vec<_>>()
                                .join("|"),
                        )?;
                    } else {
                        // Print newline
                        writeln!(&mut s)?;
                    }
                }

                writeln!(&mut s)?;
            }
            pd::Event::End(_) => {
                depth -= 1;

                if depth == 0 {
                    write!(&mut s, "{}\n\n", source.trim_end())?;
                }
            }
            _ => {}
        }
    }

    Ok(s)
}
