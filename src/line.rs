use std::io::{stdout, Write};

pub struct Line {
    pos: usize,
    str: String,
}

impl Line {
    pub fn new(str: String) -> Line {
        Line {
            pos: str.len(),
            str,
        }
    }

    pub fn new_empty() -> Line {
        Line {
            pos: 0,
            str: String::new(),
        }
    }

    pub fn add(&mut self, c: char) {
        self.str.insert(self.pos, c);
        self.pos += 1;
    }

    pub fn remove(&mut self) {
        let _ = self.str.remove(self.pos - 1);
        self.pos -= 1;
    }
}

pub struct LineList {
    pub row: usize,
    e: Vec<Line>,
}

impl LineList {
    pub fn new() -> LineList {
        LineList {
            row: 1,
            e: Vec::new(),
        }
    }

    pub fn join(&self) -> String {
        let lines: Vec<String> = self.e.iter().map(|l| l.str.clone()).collect();
        lines.join("\n")
    }

    pub fn new_line(&mut self, line: Line) {
        self.e.push(line)
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

    pub fn print_line(&self) {
        printf!(
            "{}",
            format!("\x1b[2K\r{} {}", self.row, self.e[self.row - 1].str)
        );
        printf!(
            "\r\x1b[{}G",
            self.current_pos() + format!("{}", self.row).len() + 2
        );
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
}
