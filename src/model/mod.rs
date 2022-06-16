pub mod nonogram;
pub mod board;

pub trait HVSize {
    fn h_len(&self) -> usize;

    fn v_len(&self) -> usize;
}