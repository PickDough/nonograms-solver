use std::collections::HashMap;

use crate::model::{board::Board, nonogram::Nonogram};

pub struct SimpleSolver {}

impl SimpleSolver {
    pub fn solve(&self, nonogram: Nonogram) -> Board {}
}

fn guaranteed_moves(nonogram: &Nonogram, board: &mut Board, cell_state: HashMap<Vec<i32>, bool>) {
    for i in 0..nonogram.horizontal_clues.len() {
        let h = nonogram.horizontal_clues[i];
        if cell_state[&h] {
            continue;
        }

        let clue_sum = h.iter().fold(0, |accum, item| accum + item + 1);

        if clue_sum == 0 {
            cell_state[&h] = true;
            continue;
        }

        fill_board(&mut board.state[i], clue_sum);
    }
}
// Розбивати на блоки: всі білі до перших чорних включно або хрестиків невключно. Потім дивитись максимальну кількість підказок,
// які влазять у цей проміжок і дивитись максимальний зафарбований проміжок, який можна утворити.
fn fill_board(i: &mut Vec<i32>, clue_sum: i32) {
    todo!()
}
