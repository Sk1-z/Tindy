use super::Mode;
use crate::line::{Line, LineList};
use crate::terminal::get_char;
use std::io::stdout;
use std::io::Write;

pub fn handle_insert(lines: &mut LineList) -> Mode {
    loop {
        let c: char = get_char();

        match c as usize {
            27 => {
                return Mode::Escape;
            }
            10 => {
                if lines.line_count() == lines.row {
                    lines.new_line(Line::new_empty());
                }

                if lines.current_pos() != lines.line_length() {
                    let chunk = lines.remove_chunk(lines.current_pos(), lines.line_length());
                    lines.print_line();

                    lines.row += 1;
                    for c in chunk.chars() {
                        lines.add(c);
                    }
                    lines.set_pos_relative(chunk.len(), true);
                } else {
                    lines.row += 1;
                }

                printf!("\x1b[1E");
                lines.print_line();
            }
            127 => {
                if !(lines.current_pos() == 0 && lines.row == 1) {
                    if lines.current_pos() == 0 {
                        if lines.line_length() == 0 {
                            printf!("\x1b[2K");
                            lines.remove_line();
                            lines.row -= 1;
                        } else {
                            let chunk = lines.remove_chunk(0, lines.line_length());
                            lines.print_line();

                            lines.row -= 1;
                            for c in chunk.chars() {
                                lines.add(c);
                            }
                            lines.set_pos_relative(chunk.len(), true);
                        }

                        printf!("\x1b[1F");
                        lines.print_line();
                    } else {
                        lines.remove();
                        lines.print_line();
                    }
                }
            }
            32..=126 => {
                lines.add(c);
                lines.print_line();
            }
            _ => {}
        }
    }
}
