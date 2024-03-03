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
mod mode;
mod terminal;

use line::{Line, LineList};
use mode::{escape::handle_escape, insert::handle_insert, Mode};
use std::env::args;
use std::fs::{create_dir_all, remove_file, File, OpenOptions};
use std::io::{stdout, Read, Write};
use std::path::Path;
use terminal::Terminal;

fn main() {
    let term = Terminal::new();
    term.make_raw();

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
                let file_lines: Vec<&str> = file_content.split("\n").collect();

                for i in 0..file_lines.len() {
                    lines.new_line(Line::new(String::from(file_lines[i])));
                    lines.print_line();
                    if i < file_lines.len() - 1 {
                        printf!("\n");
                        lines.row += 1
                    }
                }
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

    let mut mode = Mode::Escape;

    loop {
        match mode {
            Mode::Escape => {
                mode = handle_escape(&mut lines, (&mut file, &mut temp_file));
            }
            Mode::Insert => {
                mode = handle_insert(&mut lines);
            }
            Mode::Quit => {
                printf!("\x1b[0m");
                let joined_lines = lines.join();

                // printlnf!("\n{}", joined_lines);

                file.write_all(joined_lines.as_bytes()).unwrap();
                file.flush().unwrap();

                break;
            }
        }
    }

    remove_file(format!("/tmp/tindy/{}", file_name.to_str().unwrap())).unwrap();
    if file_name.to_str().unwrap() == ".tindy.temp" {
        remove_file(".tindy.temp").unwrap();
    }
}
