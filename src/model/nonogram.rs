pub struct Nonogram {
    pub horizontal_clues: Vec<Vec<i32>>,
    pub vertical_clues: Vec<Vec<i32>>,
}

impl super::HVSize for Nonogram {
    fn h_len(&self) -> usize {
        return self.horizontal_clues.len();
    }

    fn v_len(&self) -> usize {
        return self.vertical_clues.len();
    }
}
