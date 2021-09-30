use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_possibles::{remove_from_possibles, SudokuPossibles};
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
      }
    }
  }
}

pub fn resolve_infer(board: &mut [[SudokuCell; 9]; 9]) {
  // Por cada numero
  for n in 0..9 {
    // Recorremos todas las filas
    for z in 0..9 {
      // para comprobar si hay un solo numero para esta fila
      let mut found_in_row = false;
      let mut unique_in_row = true;
      let mut position_row = 0;
      let mut found_in_col = false;
      let mut unique_in_col = true;
      let mut position_col = 0;
      for w in 0..9 {
        if board[z][w].value == n + 1{
          found_in_row = true;
          unique_in_row = false;
        }
        if board[z][w].possibles[n as usize] {
          if found_in_row {
            unique_in_row = false;
          } else {
            found_in_row = true;
            position_row = w;
          }
        }
        if board[w][z].value ==  n + 1 {
          found_in_col = true;
          unique_in_col = false;
        }
        if board[w][z].possibles[n as usize] {
          if found_in_col {
            unique_in_col = false;
          } else {
            found_in_col = true;
            position_col = w;
          }
        }
      }
      if found_in_row && unique_in_row {
        // println!("infer row: {} {} {}", z, position_row, n +1 );
        set_guess(board, (n + 1) as u8, z, position_row)
      }
      if found_in_col && unique_in_col {
        set_guess(board, (n + 1) as u8, position_col, z)
      }
    }
    // comprobar los quartos:
    for input in 0..9 {
      let mut found = false;
      let mut unique = true;
      let mut position_x = 0;
      let mut position_y = 0;
      let sx = input / 3;
      let sy = input % 3;
      for x in 0..3 {
        for y in 0..3 {
          let mx = sx *3 + x;
          let my = sy *3 + y;
          if board[mx][my].value ==  n + 1 {
            found = true;
            unique = false;
          }
          if board[mx][my].possibles[n as usize] {
            if found {
              unique = false;
            } else {
              found = true;
              position_x = mx;
              position_y = my;
            }
          }
        }
      }
      if found && unique {
        println!("infer q: {} {} {}", position_x, position_y, n +1 );
        set_guess(board, (n + 1) as u8, position_x, position_y)
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
  for z in 0..9 {
    board[x][z].possibles[n - 1] = false;
    board[z][y].possibles[n - 1] = false;
  }
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

pub fn is_finis(board: [[SudokuCell; 9]; 9]) -> i32 {
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