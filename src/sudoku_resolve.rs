use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_resolve_util::check_if_occurrence;
use crate::sudoku_resolve_util::get_numbers_from_cell;
use crate::sudoku_resolve_util::get_range_invert_from_n;
use crate::sudoku_util::set_guess;

mod resolve_infer;
mod resolve_direct;
mod clear_board;
mod clear_by_tuples;
mod clear_by_constrain;

pub fn resolve_infer(board: &mut [[SudokuCell; 9]; 9]){
  resolve_infer::resolve_infer(board);
}

pub fn resolve_direct(board: &mut [[SudokuCell; 9]; 9]) {
  resolve_direct::resolve_direct(board);
}

pub fn clear_board(board: &mut [[SudokuCell; 9]; 9]) {
  clear_board::clear_board(board);
}

pub fn clean_by_tuples(board: &mut [[SudokuCell; 9]; 9]) {
  clear_by_tuples::clean_by_tuples(board);
}

pub fn is_finish(board: [[SudokuCell; 9]; 9]) -> i32 {
  let mut remaining = 0;
  for row in board {
    for cell in row {
      if cell.cell_type == SudokuCellType::Empty {
        remaining += 1;
      }
    }
  }
  return remaining;
}

pub fn clean_by_quarter_constrain(board: &mut [[SudokuCell; 9]; 9]) {
  // Necesitamos montar el tema de las posibilidades
  clear_by_constrain::clean_by_quarter_constrain(board);
}
