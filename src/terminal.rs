use libc::*;
use std::io::{stdin, Read};

pub fn get_char() -> char {
    stdin().bytes().next().unwrap().map(|b| b as char).unwrap()
}

pub struct Terminal {
    pub row_sz: usize,
    pub col_sz: usize,
    sz: winsize,
    canonical_mode: termios,
    raw_mode: termios,
}

impl Terminal {
    pub fn new() -> Terminal {
        let mut sz: winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let mut canonical_mode: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        let mut raw_mode: termios;

        unsafe {
            ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut sz);
            tcgetattr(STDIN_FILENO, &mut canonical_mode);
            raw_mode = canonical_mode.clone();
            raw_mode.c_lflag &= !ICANON;
            raw_mode.c_lflag &= !ECHO;
        }

        Terminal {
            row_sz: sz.ws_row as usize,
            col_sz: sz.ws_col as usize,
            sz,
            canonical_mode,
            raw_mode,
        }
    }

    pub fn make_canonical(&mut self) {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &self.canonical_mode);
        }
    }

    pub fn make_raw(&self) {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &self.raw_mode);
        }
    }

    pub fn update_size(&mut self) {
        unsafe {
            ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut self.sz);
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.make_canonical();
    }
}
