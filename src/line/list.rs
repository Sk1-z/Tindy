use super::Line;
use crate::Terminal;
use std::fs::File;
use std::io::{stdout, Read, Write};

pub struct LineList {
    pub top_row: usize,
    pub row: usize,
    e: Vec<Line>,
}

impl LineList {
    pub fn new() -> LineList {
        LineList {
            top_row: 1,
            row: 1,
            e: Vec::new(),
        }
    }

    pub fn load_from_file(&mut self, file_name: &String) {
        printf!("\x1b[2J\x1b[H");

        let reading_file = File::open(file_name);

        match reading_file {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                let file_lines: Vec<&str> = file_content.trim_end().split("\n").collect();
                file_lines
                    .iter()
                    .for_each(|line| self.new_line(Line::new(String::from(line.to_owned()))));
            }
            Err(_) => {
                self.new_line(Line::new_empty());
            }
        }
    }

    pub fn join(&self) -> String {
        let lines: Vec<String> = self.e.iter().map(|l| l.str.clone()).collect();
        lines.join("\n")
    }

    pub fn new_line(&mut self, line: Line) {
        self.e.push(line);
    }

    pub fn insert_line(&mut self, i: usize, line: Line) {
        self.e.insert(i, line)
    }

    pub fn remove_line(&mut self) {
        self.e.remove(self.row - 1);
    }

    pub fn line_length(&self) -> usize {
        self.e[self.row - 1].str.len()
    }

    pub fn line_count(&self) -> usize {
        self.e.len()
    }

    pub fn add(&mut self, c: char) {
        self.e[self.row - 1].add(c)
    }

    pub fn remove(&mut self) {
        self.e[self.row - 1].remove()
    }

    pub fn remove_chunk(&mut self, start: usize, end: usize) -> String {
        let chunk = self.e[self.row - 1]
            .str
            .get(start..end)
            .unwrap()
            .to_string();

        self.e[self.row - 1].pos = end;
        for _ in 0..(end - start) {
            self.e[self.row - 1].remove();
        }

        chunk
    }

    pub fn print_all(&mut self, term: &Terminal) {
        let s = self.row;
        for i in 0..self.e.len() - s {
            if i == term.row_sz - 1 {
                break;
            }

            self.print_line(term.col_sz);
            self.row += 1;
            printf!("\n");
        }
        self.print_line(term.col_sz);
    }

    pub fn print_all_from_top(&mut self, term: &Terminal) {
        let current_row = self.row;
        self.row = self.top_row;
        self.print_all(term);
        self.row = current_row;
    }

    pub fn print_line(&self, max_cols: usize) {
        printf!("\r\x1b[3C");
        for _ in 0..max_cols - 6 {
            printf!(" ");
        }
        printf!("\r\x1b[3C");
        let rowc = format!("{}", self.row).len();
        let padding = format!("{}", self.e.len()).len() - rowc;
        for _ in 0..padding {
            printf!(" ");
        }

        printf!(
            "{}",
            format!("\x1b[1;35m{}\x1b[0m {}", self.row, self.e[self.row - 1].str)
        );
        printf!("\r\x1b[{}G", self.current_pos() + rowc + padding + 2 + 3);
    }

    pub fn current_pos(&self) -> usize {
        self.e[self.row - 1].pos
    }

    pub fn set_pos_relative(&mut self, pos: usize, subtract: bool) {
        if subtract {
            self.e[self.row - 1].pos -= pos;
        } else {
            self.e[self.row - 1].pos += pos;
        }
    }

    pub fn reset_line_pos(&mut self) {
        self.e[self.row - 1].pos = 0;
    }
}
