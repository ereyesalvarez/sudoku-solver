use std::{thread, time};
use crate::sudoku_process::SudokuCell;
use crate::sudoku_process::SudokuCellType;

pub fn set_guess(board: &mut [[SudokuCell; 9]; 9], input: u8, x: usize, y: usize) {
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Guess;
      board[x][y].possibles = [false; 9];
    }
  }
}

pub fn set_val(board: &mut [[SudokuCell; 9]; 9], input: u8, x: usize, y: usize) {
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Fixed;
      board[x][y].possibles = [false; 9];
    }
  }
}

pub fn calculate_quarter(x: usize, y: usize) -> usize {
  let a = x / 3;
  let b = y / 3;
  return a * 3 + b;
}

pub fn _sleep_time() {
  let time = time::Duration::from_millis(3000);
  thread::sleep(time);
}

