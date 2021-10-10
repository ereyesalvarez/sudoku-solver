use std::io::{self};
use std::io::Error;
use std::io::ErrorKind;

use crate::sudoku_mock::fake;
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuCellType};

pub fn check_valid_sudoku(board: SudokuBoard) -> (bool, usize, usize) {
  // Comprueba que en las row solo se encuentre un valor de cada numero.
  for i in 0..9 {
    let row = board.board[i];
    let mut find = [false; 9];
    for j in 0..9 {
      let val = row[j];
      if !check_valid_number(val.value, &mut find) {
        return (false, i, j);
      }
    }
  }
  for i in 0..9 {
    let mut find = [false; 9];
    for j in 0..9 {
      let val = board.board[j][i];
      if !check_valid_number(val.value, &mut find) {
        return (false, i, j);
      }
    }
  }
  for x in 0..3 {
    for y in 0..3 {
      let mut find = [false; 9];
      for n in 0..3 {
        for n2 in 0..3 {
          let aux_x = x * 3 + n;
          let aux_y = y * 3 + n2;
          let val = board.board[aux_x][aux_y];
          if !check_valid_number(val.value, &mut find) {
            return (false, aux_x, aux_y);
          }
        }
      }
    }
  }
  return (true, 99, 99);
}

fn check_valid_number(value: u8, find: &mut [bool; 9]) -> bool {
  if value != 0 {
    if value > 9 {
      return false;
    }
    let position: usize = (value - 1).into();
    if find[position] {
      return false;
    }
    find[position] = true;
  }
  return true;
}

pub fn get_remaining_cells(board: SudokuBoard) -> isize {
  let mut remaining = 0;
  for row in board.board {
    for cell in row {
      if cell.is_empty() {
        remaining += 1;
      }
    }
  }
  return remaining;
}