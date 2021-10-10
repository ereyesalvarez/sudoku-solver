use std::time;
use std::time::Duration;

use crate::{sudoku_mock};
use crate::sudoku_gui::{ask_for_options, print_full_board_clean, print_full_board_clean_and_info, print_intro};
use crate::sudoku_validate::{check_valid_sudoku, get_remaining_cells};
use crate::sudoku_mock::fake;
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuOptions};
use crate::resolve::{naked};
use crate::solve_steps::{do_step, StepEnum};

pub fn game() {
  let opt = start_game();
  let mut board = load_board(opt);
  print_full_board_clean(board, opt);
  let board = do_step(board, opt, StepEnum::Pinned);
  let remaining = get_remaining_cells(board);
  let iteration = 0;
  let board = do_step(board, opt, StepEnum::LastRemain);
  print_full_board_clean_and_info(board, opt, iteration, remaining);
  let (valid, invalid_x, invalid_y) = check_valid_sudoku(board);
  if !valid {
    panic!("Invalid sudoku on pos x: {}, y: {}", invalid_x, invalid_y);
  }
  if remaining == 0 {
    return;
  }
}

pub fn start_game() -> SudokuOptions {
  print_intro();
  return ask_for_options();
}

pub fn load_board(opt: SudokuOptions) -> SudokuBoard {
  return fake();
}

