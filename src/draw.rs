use crate::Terminal;
use std::env::args;
use std::io::{stdout, Write};

pub fn clear() {
    printf!("\x1b[2J");
}

pub fn frame(term: &Terminal, move_mode: bool) {
    printf!("\x1b[H");

    let color = if move_mode {
        "\x1b[1;34;47m"
    } else {
        "\x1b[1;37;44m"
    };
    printf!("{}", color);

    for _ in 0..term.col_sz {
        printf!(" ");
    }

    for _ in 0..term.row_sz + 2 {
        printf!("\x1b[2D\x1b[1B  ");
    }

    printf!("\x1b[H\x1b[{}B", term.col_sz - 1);
    for _ in 0..term.col_sz {
        printf!(" ");
    }

    printf!("\x1b[H");
    for _ in 0..term.row_sz + 2 {
        printf!("\x1b[1B  \x1b[2D");
    }

    printf!(
        "\x1b[H  Tindy - {}\x1b[0m",
        args().collect::<Vec<String>>()[1]
    );
}
