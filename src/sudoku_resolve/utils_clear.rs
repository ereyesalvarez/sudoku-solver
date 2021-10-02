use crate::sudoku_resolve::resolve_util::{get_range_from_n, get_range_invert_from_n};
use crate::sudoku_types::{SudokuCell, SudokuCellType};

pub fn clean_by_position(x: usize, y: usize, board: &mut [[SudokuCell; 9]; 9]) {
  if board[x][y].is_filled() {
    remove_from_col_and_row(x, y, board[x][y].value as usize, board);
    remove_from_quarter(x, y, board[x][y].value as usize, board);
  }
}

fn remove_from_col_and_row(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  for z in 0..9 {
    board[x][z].possibles[n - 1] = false;
    board[z][y].possibles[n - 1] = false;
  }
}

fn remove_from_quarter(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  let sx = (x) / 3;
  let sy = (y) / 3;
  for qx in 0..3 {
    for qy in 0..3 {
      board[sx * 3 + qx][sy * 3 + qy].possibles[n - 1] = false;
    }
  }
}

pub fn remove_possible_same_axis_other_q(x: usize, y: usize, val: usize, remove_row: bool, board: &mut [[SudokuCell; 9]; 9]) {
  if(remove_row){
    for l in get_range_invert_from_n(y) {
      board[x][l].possibles[(val - 1)] = false;
    }
  } else {
    for l in get_range_invert_from_n(x) {
      board[l][y].possibles[(val - 1)] = false;
    }
  }
}

fn remove_possibles_other_q(q: usize, val: usize, pos: usize, remove_row: bool, board: &mut [[SudokuCell; 9]; 9]) {
  let start = q * 3;
  for i in start..start + 3 {
    for aux in get_range_from_n(pos) {
      if aux != pos {
        let (x, y) = if remove_row {
          (aux, i)
        } else {
          (i, aux)
        };
        if board[x][y].cell_type == SudokuCellType::Empty {
          board[x][y].possibles[(val - 1)] = false;
        }
      }
    }
  }
}

pub fn remove_possibles_from_q_other_row(q: usize, val: u8, x: usize, board: &mut [[SudokuCell; 9]; 9]) {
  remove_possibles_other_q(q, val as usize, x, true, board);
}

pub fn remove_possibles_from_q_other_col(q: usize, val: u8, y: usize, board: &mut [[SudokuCell; 9]; 9]) {
  remove_possibles_other_q(q, val as usize, y, false, board);
}

