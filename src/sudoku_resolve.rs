use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_possibles::{remove_from_possibles, SudokuPossibles};
use crate::sudoku_util::set_guess;

pub fn resolve(board: &mut [[SudokuCell; 9]; 9]) {
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
      }
    }
  }
}

pub fn clear_possibles(board: &mut [[SudokuCell; 9]; 9], possibles: &mut SudokuPossibles) {
  for x in 0..9 {
    for y in 0..9 {
      if board[x][y].cell_type == SudokuCellType::Fixed || board[x][y].cell_type == SudokuCellType::Guess {
        remove_from_possibles(x, y, board[x][y].value as usize, possibles);
        remove_from_board(x, y, board[x][y].value as usize, board);
        remove_from_quarter(x, y, board[x][y].value as usize, board);
      }
    }
  }
}

pub fn remove_from_board(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  // remove rows
  for z in 0..9 {
    board[x][z].possibles[n - 1] = false;
    board[z][y].possibles[n - 1] = false;
  }

  // remove cols
  // remove quarters
}

pub fn remove_from_quarter(x: usize, y: usize, n: usize, board: &mut [[SudokuCell; 9]; 9]) {
  let sx = (x) / 3;
  let sy = (y) / 3;
  print!("r {}: x {} y {} sx {} sy {} -> ", n - 1, x, y, sx, sy);

  for qx in 0..3 {
    for qy in 0..3 {
      print!("{}|{} ,", sx * 3 + qx, sy * 3 + qy);
      board[sx * 3 + qx][sy * 3 + qy].possibles[n - 1] = false;
    }
  }
  println!();
}

pub fn is_finis(board: [[SudokuCell; 9]; 9]) -> bool{
  for row in board {
    for cell in row {
      if cell.cell_type == SudokuCellType::Empty {
        return false;
      }
    }
  }
  return true;
}