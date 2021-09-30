use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_possibles::{remove_from_possibles, SudokuPossibles};
use crate::sudoku_util::set_guess;
use crate::sudoku_resolve_util::get_numbers_from_cell;
use crate::sudoku_resolve_util::check_if_occurrence;
use crate::sudoku_resolve_util::check_same_subsegment;
use crate::sudoku_resolve_util::get_range_from_n;

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

pub fn _unused(board: &mut [[SudokuCell; 9]; 9]) {
  // Si en una row | col | quarter
  // Tenenmos que buscar conjuntos que cuenten con el mismo numero de elementos
  // y que sean como mucho el mismo numero que los elementos que los contienen
  // p.e. si tenemos 1,7 ese 1,7 puede estar dos veces aunque haya más x,y,1, 7 o parecido
  // tambien nos vale que una n-tupla de un conjunto solo este n veces aunque acompañado de más num

  // Si un numero solo aparece en el  subconjunto de col | row en un mismo q
  // Podemos asumir
}

pub fn clean_by_tuples(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    // vamos a realizarlo para cada elemento.
    for y in 0..9 {
      if board[x][y].cell_type == SudokuCellType::Empty {
        let vector: Vec<usize> = get_numbers_from_cell(board[x][y]);
        let mut size = vector.len();
        if size > 2 {
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
                if x == mx {
                  occurrences_row += 1;
                }
                if y == my {
                  occurrences_col += 1;
                }
              }
            }
          }
          if occurrences == size as u8 {
            // Limpiar por el resto del quarto de las que no hacen match:
            clean_tuple_match(board, sx, sy, x, y);
          }
          if occurrences_row == size as u8 {
            println!("row match");
            for l in get_range_from_n(y) {
              let vector2: Vec<usize> = get_numbers_from_cell(board[x][y]);
              for n in vector2 {
                board[x][l].possibles[n - 1 as usize] = false;
              }
            }
          }
        }
      }
    }
  }
}

fn clean_tuple_match_row(board: &mut [[SudokuCell; 9]; 9], sx: usize, sy: usize, x: usize, y: usize) {
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
