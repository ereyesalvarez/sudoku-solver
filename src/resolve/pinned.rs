use crate::board_util::get_candidates_from_cell;
use crate::sudoku_types::SudokuBoard;

pub(super) fn pinned(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  // Pinned for column && row
  let mut hits = 0;
  for x in 0..9 {
    let mut last_pos = 99;
    let mut is_already_appear = false;
    let mut abort = false;
    for y in 0..9 {
      for i in 0..9 {
        if board.get_cell(x, y).candidates[i] {
          if is_already_appear {
            is_already_appear = false;
            abort = false;
            break;
          } else {
            is_already_appear = false;
            last_pos = i;
          }
        }
      }
      if is_already_appear && !abort {
        board.set_guess(last_pos + 1, x, y);
        hits += 1;
      }
    }
  }
  return (board, hits);
}

