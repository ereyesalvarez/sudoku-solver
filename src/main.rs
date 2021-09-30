use crate::sudoku_possibles::reset_possibles;
use crate::sudoku_util::{print_full_board, print_possibles};

mod sudoku_util;
mod sudoku_input;
mod sudoku_process;
mod sudoku_game;
mod sudoku_mock;
mod sudoku_resolve;
mod sudoku_possibles;

fn main() {
  sudoku_game::print_intro();
  let mut board = sudoku_mock::fake();
  print!("{esc}c", esc = 27 as char);
  let mut possibles = reset_possibles();
  print_full_board(board);
  sudoku_util::sleep_time();
  loop {
    sudoku_resolve::clear_possibles(&mut board, &mut possibles);
    print_full_board(board);
    sudoku_resolve::resolve(&mut board);
    sudoku_util::sleep_time();
  }
}

#[cfg(test)]
mod sudoku_test;
