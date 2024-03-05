pub mod list;

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
