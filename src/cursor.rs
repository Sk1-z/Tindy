use crate::{stdout, Write};

pub fn home() {
    printf!("\x1b[3;1H")
}
