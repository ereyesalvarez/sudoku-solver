use crate::sudoku_resolve::utils_clear::clean_by_position;
use crate::sudoku_types::{SudokuCell};

pub fn clear_board(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    for y in 0..9 {
      clean_by_position(x, y, board);
    }
  }
}
