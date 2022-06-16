pub struct Board {
    pub state: Vec<Vec<i32>>,
}

impl super::HVSize for Board {
    fn h_len(&self) -> usize {
        return self.state.len();
    }

    fn v_len(&self) -> usize {
        return self.state[0].len();
    }
}
