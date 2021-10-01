use crate::sudoku_mock::fake;
use crate::sudoku_possibles::{print_possibles, remove_from_possibles, reset_possibles, SudokuPossibles};
use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_resolve_util::{get_range_from_n, get_range_invert_from_n, is_cell_possible_present};

pub fn clean_by_quarter_constrain(board: &mut [[SudokuCell; 9]; 9]) {
  // Identificar donde hay en una fila / columna un numero contraido a un solo quarter
  // Para cada columna
  for y in 0..9 {
    println!(" --  ");
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
            if board[x][y].possibles[(val - 1) as usize] {
              print!("{} - {}{} -> {} ",val, x, y, board[x][y].possibles[(val - 1) as usize]);
              for i in 0..9 {
                if board[x][y].possibles[i] {
                  print!("{} ", i + 1);
                }
              }
              println!();
            }
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
        println!("cleaning constrain y {} q {}, val {}, ", y, 0, val);
        remove_possibles_from_q_other_col(0, val, y, board);
      }
      if !abort && !in_section_1 && in_section_2 && !in_section_3 {
        // Esta solo en la 2 parte
        println!("cleaning constrain y {} q {}, val {}, ", y, 1, val);
        remove_possibles_from_q_other_col(1, val, y, board);
      }
      if !abort && !in_section_1 && !in_section_2 && in_section_3 {
        // Esta solo en la 3 parte
        println!("cleaning constrain y {} q {}, val {}, ", y, 2, val);
        remove_possibles_from_q_other_col(2, val, y, board);
      }
    }
  }
}


pub fn remove_possibles_from_q_other_col(q: usize, val: u8, y: usize, board: &mut [[SudokuCell; 9]; 9]) {
  let start_x = q * 3;
  print!("delete {}... ", val);
  for x in start_x..start_x + 3 {
    for aux_y in get_range_from_n(y) {
      if aux_y != y && board[x][aux_y].cell_type == SudokuCellType::Empty {
        if board[x][aux_y].possibles[(val -1) as usize] {
          print!("{} {}", x, aux_y);
        }
        board[x][aux_y].possibles[(val - 1) as usize] = false;
      }
    }
  }
  println!();
}

pub fn remove_possibles_from_other_sement_col(q: usize, val: u8, y: usize, board: &mut [[SudokuCell; 9]; 9]) {
  for aux_y in get_range_from_n(y) {
    for x in get_range_invert_from_n(q * 3) {
      if aux_y != y && board[x][y].cell_type == SudokuCellType::Empty {
        board[x][y].possibles[(val - 1) as usize] = false;
      }
    }
  }
}
