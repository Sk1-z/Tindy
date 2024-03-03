use super::Mode;
use crate::line::LineList;
use crate::terminal::get_char;
use std::fs::File;
use std::io::stdout;
use std::io::Write;

pub fn handle_escape(lines: &mut LineList, files: (&mut File, &mut File)) -> Mode {
    let (file, temp_file) = files;

    loop {
        let c: char = get_char();

        match c as usize {
            113 => return Mode::Quit,
            105 => return Mode::Insert,
            65 | 107 => {
                if lines.row != 1 {
                    lines.row -= 1;

                    printf!("\x1b[1F");
                    lines.print_line();
                }

                if c as usize == 65 {
                    return Mode::Insert;
                }
            }
            66 | 106 => {
                if lines.line_count() != lines.row {
                    printf!("\x1b[1E");

                    lines.row += 1;
                    lines.print_line();
                }

                if c as usize == 66 {
                    return Mode::Insert;
                }
            }
            67 | 108 => {
                if lines.current_pos() != lines.line_length() {
                    lines.set_pos_relative(1, false);
                    lines.print_line();
                }

                if c as usize == 67 {
                    return Mode::Insert;
                }
            }
            68 | 104 => {
                if lines.current_pos() != 0 {
                    lines.set_pos_relative(1, true);
                    lines.print_line();
                }

                if c as usize == 68 {
                    return Mode::Insert;
                }
            }
            _ => {}
        }
    }
}
