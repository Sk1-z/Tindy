use crate::cursor;
use crate::draw;
use crate::Line;
use crate::LineList;
use crate::Terminal;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::rc::Rc;

pub struct ActionHandler {
    term: Rc<RefCell<Terminal>>,
    lines: Rc<RefCell<LineList>>,
}

impl ActionHandler {
    pub fn new(term: Rc<RefCell<Terminal>>, lines: Rc<RefCell<LineList>>) -> ActionHandler {
        ActionHandler { term, lines }
    }

    pub fn start_server(file_name: String) {
        let script = read_to_string(format!(
            "{}/.tindy/py/server.py",
            std::env::var_os("HOME").unwrap().to_str().unwrap()
        ))
        .unwrap();

        Python::with_gil(|py| {
            let func: Py<PyAny> = PyModule::from_code(py, &script, "python/server.py", "py.server")
                .unwrap()
                .getattr("start_server")
                .unwrap()
                .into();

            let args = PyTuple::new(py, &[file_name]);
            _ = func.call1(py, args);
        });
    }

    pub fn save(&self, file_name: &String) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .unwrap();

        let joined_lines = self.lines.borrow().join();

        file.write_all(joined_lines.as_bytes()).unwrap();
        file.flush().unwrap();
    }

    pub fn new_line(&mut self, move_mode: bool) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        let current_pos = lines.current_pos();
        let line_length = lines.line_length();

        let chunk = lines.remove_chunk(current_pos, line_length);
        if lines.line_count() == lines.row {
            lines.new_line(Line::new_empty());
        } else {
            let row = lines.row;
            lines.insert_line(row, Line::new_empty());
        }
        lines.row += 1;

        lines.reset_line_pos();
        for c in chunk.chars() {
            lines.add(c);
        }
        lines.reset_line_pos();

        draw::clear();
        draw::frame(&term, move_mode);
        cursor::home();
        if lines.row == lines.top_row + term.row_sz {
            lines.top_row = lines.row;
            lines.print_all(&term);

            cursor::home();
            lines.row = lines.top_row;
            lines.print_line(term.col_sz);
        } else {
            lines.print_all_from_top(&term);
            cursor::home();
            let move_sz = (lines.row % term.row_sz) - 1;
            if move_sz != 0 {
                printf!("\x1b[{}E", move_sz);
            }
        }

        lines.print_line(term.col_sz);
    }

    pub fn delete(&mut self, move_mode: bool) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        if !(lines.current_pos() == 0 && lines.row == 1) {
            if lines.current_pos() == 0 {
                let line_length = lines.line_length();
                let chunk = lines.remove_chunk(0, line_length);
                lines.remove_line();

                lines.row -= 1;
                for c in chunk.chars() {
                    lines.add(c);
                }
                lines.set_pos_relative(chunk.len(), true);

                draw::clear();
                draw::frame(&term, move_mode);
                cursor::home();
                if lines.row + 1 == lines.top_row && lines.row != 1 {
                    lines.top_row -= term.row_sz;
                    lines.row = lines.top_row;
                    lines.print_all(&term);
                } else {
                    lines.print_all_from_top(&term);
                    cursor::home();
                    let move_sz = (lines.row % term.row_sz) - 1;
                    if move_sz != 0 {
                        printf!("\x1b[{}E", move_sz);
                    }
                }
                lines.print_line(term.col_sz);
            } else {
                lines.remove();
                lines.print_line(term.col_sz);
            }
        }
    }

    pub fn add_char(&mut self, c: char) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        lines.add(c);
        lines.print_line(term.col_sz);
    }
}
