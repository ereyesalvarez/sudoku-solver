mod sudoku_input;
mod sudoku_process;
mod sudoku_game;
mod sudoku_mock;
mod sudoku_resolve;
mod sudoku_resolve_util;
mod sudoku_util;
mod sudoku_gui;
mod sudoku_possibles;

fn main() {
  sudoku_game::play();
}


#[cfg(test)]
mod sudoku_test;
