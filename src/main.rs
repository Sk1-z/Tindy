#[macro_export]
macro_rules! printf {
    ($($fmt:tt)*) => {{
        print!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! printlnf {
    ($($fmt:tt)*) => {{
        println!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

mod draw;
mod line;
mod terminal;

use line::{Line, LineList};
use std::env::args;
use std::fs::{create_dir_all, remove_file, File, OpenOptions};
use std::io::{stdout, Read, Write};
use std::path::Path;
use terminal::*;

fn main() {
    let term = Terminal::get();
    term.make_raw();
    // printf!("\x1b[2J\x1b[H");
    // printlnf!("1");
    // for _ in 0..term.row_sz - 1 {
    //     printlnf!("2");
    // }
    //
    // loop {}

    let mut file_name: &Path;
    let argv: Vec<String> = args().collect();
    if argv.len() == 2 {
        file_name = Path::new(&argv[1])
    } else {
        file_name = Path::new(".tindy.temp")
    }

    let mut lines = LineList::new();

    {
        printf!("\x1b[2J\x1b[H");

        let reading_file = File::open(file_name);

        match reading_file {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                let file_lines: Vec<&str> = file_content.trim_end().split("\n").collect();
                file_lines
                    .iter()
                    .for_each(|line| lines.new_line(Line::new(String::from(line.to_owned()))));

                lines.print_all(term.row_sz);
                printf!("\x1b[H");
                lines.row = 1;
                lines.reset_line_pos();
                lines.print_line();
            }
            Err(_) => {
                lines.new_line(Line::new_empty());
                lines.print_line();
            }
        }
    }

    let mut open_options = OpenOptions::new();
    open_options.read(true).write(true).create(true);

    create_dir_all("/tmp/tindy").unwrap();
    let mut file = open_options.open(file_name).unwrap();
    let mut temp_file = open_options
        .open(format!("/tmp/tindy/{}", file_name.to_str().unwrap()))
        .unwrap();

    loop {
        let c = get_char();

        match c as usize {
            // ctrl-e or  ctrl-w
            5 | 23 => {
                printf!("\n\x1b[2J\x1b[0m");
                let joined_lines = lines.join();

                // printlnf!("\n{}", joined_lines);

                file.write_all(joined_lines.as_bytes()).unwrap();
                file.flush().unwrap();

                break;
            }
            // Handle arrow presses which are sent as three characters or exit
            // 27 ESC -> 91 [ -> A, B, C, or D
            27 => {
                // Discard [
                _ = get_char();
                match get_char() as usize {
                    // up arrow
                    65 => {
                        if lines.row == lines.top_row && lines.row != 1 {
                            printf!("\x1b[2J\x1b[H");

                            lines.top_row -= term.row_sz;
                            lines.row = lines.top_row;

                            lines.print_all(term.row_sz);
                        } else {
                            if lines.row != 1 {
                                lines.row -= 1;

                                printf!("\x1b[1F");
                                lines.print_line();
                            }
                        }
                    }
                    // down arrow
                    66 => {
                        if lines.row == lines.top_row + term.row_sz - 1 {
                            printf!("\x1b[2J\x1b[H");

                            lines.row += 1;
                            lines.top_row = lines.row;

                            lines.print_all(term.row_sz);
                            printf!("\x1b[H");

                            lines.row = lines.top_row;
                            lines.print_line();
                        } else {
                            if lines.line_count() != lines.row {
                                lines.row += 1;

                                printf!("\x1b[1E");
                                lines.print_line();
                            }
                        }
                    }
                    // right arrow
                    67 => {
                        if lines.current_pos() != lines.line_length() {
                            lines.set_pos_relative(1, false);
                            lines.print_line();
                        }
                    }
                    // left arrow
                    68 => {
                        if lines.current_pos() != 0 {
                            lines.set_pos_relative(1, true);
                            lines.print_line();
                        }
                    }
                    _ => {}
                }
            }
            // Enter
            10 => {
                let chunk = lines.remove_chunk(lines.current_pos(), lines.line_length());
                if lines.line_count() == lines.row {
                    lines.new_line(Line::new_empty());
                } else {
                    lines.insert_line(lines.row, Line::new_empty());
                }
                lines.row += 1;

                lines.reset_line_pos();
                for c in chunk.chars() {
                    lines.add(c);
                }
                lines.reset_line_pos();

                printf!("\x1b[2J\x1b[H");
                if lines.row == lines.top_row + term.row_sz {
                    lines.top_row = lines.row;
                    lines.print_all(term.row_sz);

                    printf!("\x1b[H");
                    lines.row = lines.top_row;
                    lines.print_line();
                } else {
                    lines.print_all_from_top(term.row_sz);
                    printf!("\x1b[H");
                    let move_sz = (lines.row % term.row_sz) - 1;
                    if move_sz != 0 {
                        printf!("\x1b[{}E", move_sz);
                    }
                }

                lines.print_line();
            }
            // Backspace or ctrl-x
            127 | 24 => {
                if !(lines.current_pos() == 0 && lines.row == 1) {
                    if lines.current_pos() == 0 {
                        let chunk = lines.remove_chunk(0, lines.line_length());
                        lines.remove_line();

                        lines.row -= 1;
                        for c in chunk.chars() {
                            lines.add(c);
                        }
                        lines.set_pos_relative(chunk.len(), true);

                        printf!("\x1b[2J\x1b[H");
                        if lines.row + 1 == lines.top_row && lines.row != 1 {
                            lines.top_row -= term.row_sz;
                            lines.row = lines.top_row;
                            lines.print_all(term.row_sz);
                        } else {
                            lines.print_all_from_top(term.row_sz);
                            printf!("\x1b[H");
                            let move_sz = (lines.row % term.row_sz) - 1;
                            if move_sz != 0 {
                                printf!("\x1b[{}E", move_sz);
                            }
                        }
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

    remove_file(format!("/tmp/tindy/{}", file_name.to_str().unwrap())).unwrap();
    if file_name.to_str().unwrap() == ".tindy.temp" {
        remove_file(".tindy.temp").unwrap();
    }
}
