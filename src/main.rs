mod sudoku_util;
mod sudoku_input;
mod sudoku_process;
mod sudoku_game;
mod sudoku_mock;

fn main() {
  sudoku_game::print_intro();
  let mut board = sudoku_mock::fake();
  print!("{esc}c", esc = 27 as char);
  sudoku_util::print(board);
}

#[cfg(test)]
mod sudoku_test;
