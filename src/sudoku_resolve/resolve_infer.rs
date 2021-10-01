use crate::sudoku_process::SudokuCell;
use crate::sudoku_util::set_guess;

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
        if board[z][w].value == n + 1 {
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
        if board[w][z].value == n + 1 {
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
          let mx = sx * 3 + x;
          let my = sy * 3 + y;
          if board[mx][my].value == n + 1 {
            found = true;
            unique = false;
          } else if board[mx][my].possibles[n as usize] {
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
        set_guess(board, (n + 1) as u8, position_x, position_y)
      }
    }
  }
}