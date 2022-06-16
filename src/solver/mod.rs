mod solver;

use crate::model::nonogram::Nonogram;

pub trait Reader {
    fn read(&self) -> Nonogram;
}
