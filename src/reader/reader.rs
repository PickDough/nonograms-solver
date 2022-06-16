use crate::model::nonogram::Nonogram;

impl crate::solver::Reader for (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    fn read(&self) -> Nonogram {
        return Nonogram {
            horizontal_clues: self.0.to_vec(),
            vertical_clues: self.1.to_vec(),
        };
    }
}

impl crate::solver::Reader for (&[&[i32]], &[&[i32]]) {
    fn read(&self) -> Nonogram {
        return Nonogram {
            horizontal_clues: self.0.iter().map(|c| c.to_vec()).collect(),
            vertical_clues: self.1.iter().map(|c| c.to_vec()).collect(),
        };
    }
}
