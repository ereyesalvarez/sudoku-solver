use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_resolve::resolve_util::{check_if_occurrence, get_numbers_from_cell, get_range_invert_from_n};

pub fn clean_by_tuples(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    // vamos a realizarlo para cada elemento.
    for y in 0..9 {
      if board[x][y].cell_type == SudokuCellType::Empty {
        let vector: Vec<usize> = get_numbers_from_cell(board[x][y]);
        let size = vector.len();
        if size > 1 {
          // Comprobar repetición en su cuadrante.
          let mut occurrences: u8 = 0;
          let mut occurrences_row: u8 = 0;
          let mut occurrences_col: u8 = 0;
          let sx = (x) / 3;
          let sy = (y) % 3;
          // Este es el cuadrante
          for qx in 0..3 {
            for qy in 0..3 {
              let mx = sx * 3 + qx;
              let my = sy * 3 + qy;
              let is_match = check_if_occurrence(board[mx][my], &vector);
              if is_match {
                occurrences += 1;
              }
            }
          }
          for y_row in 0..9 {
            let is_match = check_if_occurrence(board[x][y_row], &vector);
            if is_match {
              occurrences_row += 1;
            }
          }
          for x_col in 0..9 {
            let is_match = check_if_occurrence(board[x_col][y], &vector);
            if is_match {
              occurrences_col += 1;
            }
          }

          if occurrences == size as u8 {
            // Limpiar por el resto del quarto de las que no hacen match:
            clean_tuple_match(board, sx, sy, x, y);
          }

          if occurrences_row == size as u8 {
            for l in get_range_invert_from_n(y) {
              let vector2: Vec<usize> = get_numbers_from_cell(board[x][y]);
              let is_match = check_if_occurrence(board[x][l], &vector);
              if !is_match {
                for n in vector2 {
                  board[x][l].possibles[n - 1 as usize] = false;
                }
              }
            }
          }

          if occurrences_col == size as u8 {
            for l in get_range_invert_from_n(x) {
              let vector2: Vec<usize> = get_numbers_from_cell(board[x][y]);
              let is_match = check_if_occurrence(board[l][y], &vector);
              if !is_match {
                for n in vector2 {
                  board[l][y].possibles[n - 1 as usize] = false;
                }
              }
            }
          }
        }
      }
    }
  }
}
fn clean_tuple_match(board: &mut [[SudokuCell; 9]; 9], sx: usize, sy: usize, x: usize, y: usize) {
  // Limpiar por el resto del quarto de las que no hacen match:
  for qx in 0..3 {
    for qy in 0..3 {
      let mx = sx * 3 + qx;
      let my = sy * 3 + qy;
      let vector: Vec<usize> = get_numbers_from_cell(board[x][y]);
      let is_match = check_if_occurrence(board[mx][my], &vector);
      if !is_match {
        let vector2: Vec<usize> = get_numbers_from_cell(board[x][y]);
        for n in vector2 {
          board[mx][my].possibles[n - 1 as usize] = false;
        }
      }
    }
  }
  // Comprobar columnas y filas
}

