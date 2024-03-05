use crate::cursor;
use crate::draw;
use crate::LineList;
use crate::Terminal;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

pub struct MovementHandler {
    term: Rc<RefCell<Terminal>>,
    lines: Rc<RefCell<LineList>>,
}

impl MovementHandler {
    pub fn new(term: Rc<RefCell<Terminal>>, lines: Rc<RefCell<LineList>>) -> MovementHandler {
        MovementHandler { term, lines }
    }

    pub fn handle_up(&mut self, move_mode: bool) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        if lines.row == lines.top_row && lines.row != 1 {
            draw::clear();
            draw::frame(&term, move_mode);
            cursor::home();

            lines.top_row -= term.row_sz;
            lines.row = lines.top_row;

            lines.print_all(term.row_sz);
        } else {
            if lines.row != 1 {
                lines.row -= 1;

                printf!("\x1b[1F");
                lines.print_line(term.row_sz);
            }
        }
    }
    pub fn handle_down(&mut self, move_mode: bool) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        if lines.row == lines.top_row + term.row_sz - 1 {
            draw::clear();
            draw::frame(&term, move_mode);
            cursor::home();

            lines.row += 1;
            lines.top_row = lines.row;

            lines.print_all(term.row_sz);
            cursor::home();

            lines.row = lines.top_row;
            lines.print_line(term.row_sz);
        } else {
            if lines.line_count() != lines.row {
                lines.row += 1;

                printf!("\x1b[1E");
                lines.print_line(term.row_sz);
            }
        }
    }
    pub fn handle_right(&mut self) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        if lines.current_pos() != lines.line_length() {
            lines.set_pos_relative(1, false);
            lines.print_line(term.row_sz);
        }
    }
    pub fn handle_left(&mut self) {
        let term = self.term.borrow();
        let mut lines = self.lines.borrow_mut();

        if lines.current_pos() != 0 {
            lines.set_pos_relative(1, true);
            lines.print_line(term.row_sz);
        }
    }
}
