use crate::sudoku_resolve::resolve_util::{get_range_from_n, is_cell_possible_present};
use crate::sudoku_resolve::utils_clear::{remove_possibles_from_q_other_col, remove_possibles_from_q_other_row};
use crate::sudoku_types::{SudokuCell, SudokuCellType};

pub fn clean_by_quarter_constrain_col(board: &mut [[SudokuCell; 9]; 9]) {
  // Identificar donde hay en una fila / columna un numero contraido a un solo quarter
  // Para cada columna
  for y in 0..9 {
    // Si dentro de una columna vemos numeros que solo estan presentes en una seccion
    // Podemos hacerlo por seccion
    for val in 1..10 {
      let mut in_section_1 = false;
      let mut in_section_2 = false;
      let mut in_section_3 = false;
      let mut abort = false;

      for section in 0..3 {
        for x_section in 0..3 {
          let partial = section * 3;
          let x = partial + x_section;
          if board[x][y].value == val {
            abort = true;
            break;
          }
          if board[x][y].cell_type == SudokuCellType::Empty {
            if is_cell_possible_present(board[x][y], val as isize) {
              match section {
                0 => in_section_1 = true,
                1 => in_section_2 = true,
                2 => in_section_3 = true,
                _ => {}
              }
            }
          }
        }
      }
      if !abort && in_section_1 && !in_section_2 && !in_section_3 {
        // Esta solo en la 1 parte
        remove_possibles_from_q_other_col(0, val, y, board);
      }
      if !abort && !in_section_1 && in_section_2 && !in_section_3 {
        // Esta solo en la 2 parte
        remove_possibles_from_q_other_col(1, val, y, board);
      }
      if !abort && !in_section_1 && !in_section_2 && in_section_3 {
        // Esta solo en la 3 parte
        remove_possibles_from_q_other_col(2, val, y, board);
      }
    }
  }
}

pub fn clean_by_quarter_constrain_row(board: &mut [[SudokuCell; 9]; 9]) {
  // Identificar donde hay en una fila / columna un numero contraido a un solo quarter
  // Para cada fila
  for x in 0..9 {
    // Si dentro de una fila vemos numeros que solo estan presentes en una seccion
    // Podemos hacerlo por seccion
    for val in 1..10 {
      let mut in_section_1 = false;
      let mut in_section_2 = false;
      let mut in_section_3 = false;
      let mut abort = false;

      for section in 0..3 {
        for y_section in 0..3 {
          let partial = section * 3;
          let y = partial + y_section;
          if board[x][y].value == val {
            abort = true;
            break;
          }
          if board[x][y].cell_type == SudokuCellType::Empty {
            if is_cell_possible_present(board[x][y], val as isize) {
              match section {
                0 => in_section_1 = true,
                1 => in_section_2 = true,
                2 => in_section_3 = true,
                _ => {}
              }
            }
          }
        }
      }
      if !abort && in_section_1 && !in_section_2 && !in_section_3 {
        // Esta solo en la 1 parte
        remove_possibles_from_q_other_row(0, val, x, board);
      }
      if !abort && !in_section_1 && in_section_2 && !in_section_3 {
        // Esta solo en la 2 parte
        remove_possibles_from_q_other_row(1, val, x, board);
      }
      if !abort && !in_section_1 && !in_section_2 && in_section_3 {
        // Esta solo en la 3 parte
        remove_possibles_from_q_other_row(2, val, x, board);
      }
    }
  }
}

