use crate::board_util::{get_box_range_from_pos, get_range_of_box};
use crate::sudoku_types::{Position, SudokuBoard};

pub(super) fn last_remaining(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let (board, hits) = last_remaining_in_line(board);
  let (board, hits2) = last_remaining_in_box(board);
  return (board, hits2);
}

fn last_remaining_in_box(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let mut hits = 0;
  let mut last_pos: Position = Position { x: 99, y: 99 };
  let mut appear = false;
  let mut abort = false;
  for q in 0..9 {
    for i in 0..9 {
      let box_range = get_range_of_box(q);
      for x in box_range.0 {
        for y in box_range.1 {
          if board.get_cell(x, y).candidates[i] {
            if appear {
              abort = true;
              break;
            } else {
              appear = true;
              last_pos = Position { x, y };
            }
          }
        }
      }
      if appear && !abort {
        board.set_guess(i + 1, last_pos.x, last_pos.y);
      }
    }
  }
  return (board, hits);
}


/// El unico en una fila o en una columna
fn last_remaining_in_line(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  // Pinned for column && row
  let mut hits = 0;
  for axis_a in 0..9 {
    for i in 0..9 {
      let mut last_pos_row = 99;
      let mut last_pos_col = 99;
      let mut appear_row = false;
      let mut appear_col = false;
      let mut abort_row = false;
      let mut abort_col = false;
      for axis_b in 0..9 {
        let row_cell = board.get_cell(axis_a, axis_b);

        if row_cell.candidates[i] {
          if appear_row {
            abort_row = true;
          } else {
            appear_row = true;
            last_pos_row = axis_b;
          }
        }
        let col_cell = board.get_cell(axis_b, axis_a);
        if col_cell.candidates[i] {
          if appear_col {
            abort_col = true;
          } else {
            appear_col = true;
            last_pos_col = axis_b;
          }
        }
      }
      if appear_row && !abort_row {
        board.set_guess(i + 1, axis_a, last_pos_row);
        hits += 1;
      }
      if appear_col && !abort_col {
        board.set_guess(i + 1, last_pos_col, axis_a);
        hits += 1;
      }
    }
  }
  return (board, hits);
}
