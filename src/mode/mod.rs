pub mod escape;
pub mod insert;

pub enum Mode {
    Escape,
    Insert,
    Quit,
}
