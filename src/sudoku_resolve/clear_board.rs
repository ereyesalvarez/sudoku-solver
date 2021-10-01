use crate::sudoku_process::{SudokuCell, SudokuCellType};

pub fn clear_board(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    for y in 0..9 {
      if board[x][y].cell_type == SudokuCellType::Fixed || board[x][y].cell_type == SudokuCellType::Guess {
        remove_from_board(x, y, board[x][y].value as usize, board);
        remove_from_quarter(x, y, board[x][y].value as usize, board);
      }
    }
  }
}

pub fn remove_from_board(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  for z in 0..9 {
    board[x][z].possibles[n - 1] = false;
    board[z][y].possibles[n - 1] = false;
  }
}

pub fn remove_from_quarter(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  let sx = (x) / 3;
  let sy = (y) / 3;
  for qx in 0..3 {
    for qy in 0..3 {
      board[sx * 3 + qx][sy * 3 + qy].possibles[n - 1] = false;
    }
  }
}