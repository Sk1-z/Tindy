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

mod cursor;
mod draw;
mod handle;
mod line;
mod terminal;

use handle::*;
use line::{list::LineList, Line};
use std::cell::RefCell;
use std::env::args;
use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::io::{stdout, Write};
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use terminal::*;

fn main() {
    let term = Terminal::get();
    term.make_raw();

    let file_name: &Path;
    let mut argv: Vec<String> = args().collect();
    if argv.len() == 2 {
        argv[1].push_str(".md");
        file_name = Path::new(&argv[1]);
    } else {
        printlnf!("\x1b[1;91m[ERROR]\x1b[0m Must pass a file name. If it does not exist it will be created.");
        exit(1);
    }

    let mut move_mode = false;

    let mut lines = LineList::new();
    lines.load_from_file(file_name);

    draw::clear();
    draw::frame(&term, move_mode);

    cursor::home();
    lines.print_all(term.row_sz);

    cursor::home();
    lines.row = 1;
    lines.reset_line_pos();
    lines.print_line(term.row_sz);

    let mut open_options = OpenOptions::new();
    open_options.read(true).write(true).create(true);

    create_dir_all("/tmp/tindy").unwrap();
    let mut file = open_options.open(file_name).unwrap();

    let term_rc: Rc<RefCell<Terminal>> = Rc::new(RefCell::new(term));
    let lines_rc: Rc<RefCell<LineList>> = Rc::new(RefCell::new(lines));

    let mut movement = movement::MovementHandler::new(Rc::clone(&term_rc), Rc::clone(&lines_rc));
    let mut action = action::ActionHandler::new(Rc::clone(&term_rc), Rc::clone(&lines_rc));

    loop {
        let c = get_char();

        match c as usize {
            // Exit
            // ctrl-e
            5 => {
                printf!("\n\x1b[2J\x1b[0m");
                break;
            }
            // Write to file
            // ctrl-w
            23 => action.save(&mut file),
            // Movement mode
            // ctrl - m
            1 => {
                if move_mode {
                    move_mode = false;
                    printf!("\x1b[s");
                    draw::frame(&term_rc.borrow(), move_mode);
                    printf!("\x1b[u");
                } else {
                    move_mode = true;
                    printf!("\x1b[s");
                    draw::frame(&term_rc.borrow(), move_mode);
                    printf!("\x1b[u");
                }
            }
            // Handle arrow presses which are sent as three characters or exit
            // 27 ESC -> 91 [ -> A, B, C, or D
            27 => {
                // Discard [
                _ = get_char();
                match get_char() as usize {
                    // up arrow
                    65 => movement.handle_up(move_mode),

                    // down arrow
                    66 => movement.handle_down(move_mode),

                    // right arrow
                    67 => movement.handle_right(),

                    // left arrow
                    68 => movement.handle_left(),
                    _ => {}
                }
            }
            // Enter
            10 => action.new_line(move_mode),
            // Backspace or ctrl-x
            127 | 24 => action.delete(move_mode),
            32..=126 => {
                if move_mode {
                    match c as usize {
                        // k
                        107 => movement.handle_up(move_mode),

                        // j
                        106 => movement.handle_down(move_mode),

                        // l
                        108 => movement.handle_right(),

                        // h
                        104 => movement.handle_left(),
                        _ => action.add_char(c),
                    }
                } else {
                    action.add_char(c)
                }
            }
            _ => {}
        }
    }
}
