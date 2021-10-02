use crate::sudoku_resolve::utils_clear::clean_by_position;
use crate::sudoku_types::SudokuCell;
use crate::sudoku_util::set_guess;

pub fn resolve_direct(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    for y in 0..9 {
      let mut last = 99;
      let mut keep = true;
      for n in 0..9 {
        if keep && board[x][y].possibles[n] {
          if last == 99 {
            last = n;
          } else {
            keep = false;
          }
        }
      }
      if keep && last != 99 {
        set_guess(board, (last + 1) as u8, x, y);
        clean_by_position(x, y, board);
      }
    }
  }
}