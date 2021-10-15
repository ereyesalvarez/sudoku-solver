// ToDo:

use crate::board_util::{get_inverse_range_from_box, get_inverse_range_from_pos, get_range_from_pos, get_range_of_box};
use crate::sudoku_types::{LineType, SudokuBoard, SudokuCell};

#[derive(Copy, Clone)]
pub struct IRSectionsHelper {
  pub sec1: bool,
  pub sec2: bool,
  pub sec3: bool,
  pub abort: bool,
}

impl IRSectionsHelper {
  pub(crate) fn new() -> IRSectionsHelper {
    IRSectionsHelper { sec1: false, sec2: false, sec3: false, abort: false }
  }
  pub(crate) fn ony_sec(&self, sec: usize) -> bool {
    match sec {
      0 => return !self.abort && self.sec1 && !self.sec2 && !self.sec3,
      1 => return !self.abort && !self.sec1 && self.sec2 && !self.sec3,
      2 => return !self.abort && !self.sec1 && !self.sec2 && self.sec3,
      _ => panic!("Invalid section")
    }
  }
}

/**
Si un numero aparece dentro de la misma caja solo en la misma fila o columna:
podemos elimnar ese numero como candidato en el resto de la fila o columna
Ademas podemos eliminar de la caja las los numeros de las otras filas/columnas.
 */

pub(super) fn intersection_removal(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let hits = 0;
  for n_line in 0..9 {
    for val in 1..10 {
      let mut col_sections = IRSectionsHelper::new();
      let mut row_sections = IRSectionsHelper::new();
      // Para cada sección de la caja
      for n_box in 0..3 {
        // para cada celda de la sección
        for cell_on_box in 0..3 {
          let axis_b = (n_box * 3) + cell_on_box;
          row_sections = check_if_candidate_present(board.get_cell(n_line, axis_b), val, n_box, row_sections);
          col_sections = check_if_candidate_present(board.get_cell(axis_b, n_line), val, n_box, col_sections);
          if col_sections.abort && row_sections.abort {
            break;
          }
        }
      }
      for sec in 0..3 {
        if col_sections.ony_sec(sec) {
          board = remove_candidates_from_same_box_other_lines(board, val, n_line, sec, LineType::Col);
        }
        if row_sections.ony_sec(sec) {
          board = remove_candidates_from_same_box_other_lines(board, val, n_line, sec, LineType::Row);
        }
      }
    }
  }
  (board, hits)
}

pub fn check_if_candidate_present(cell: SudokuCell, val: u8, section: usize, mut sections: IRSectionsHelper) -> IRSectionsHelper {
  if cell.is_filled() {
    if cell.value == val {
      sections.abort = true;
    }
  } else {
    if cell.is_candidate_present(val as usize) {
      match section {
        0 => sections.sec1 = true,
        1 => sections.sec2 = true,
        2 => sections.sec3 = true,
        _ => {}
      }
    }
  }
  return sections;
}


fn remove_candidates_from_same_box_other_lines(mut board: SudokuBoard, number: u8, n_line: usize, n_box: usize, line_type: LineType) -> SudokuBoard {
  print!("remove: n {}, line {} box {} ", number, n_line, n_box);
  println!("{:?}", line_type);

  for axis_a in get_range_from_pos(n_line) {
    if axis_a != n_line {
      for axis_b in n_box * 3 ..n_box * 3 + 3 {
        match line_type {
          LineType::Row => {
            board.board[axis_a][axis_b].candidates_remove(number as usize);
          }
          LineType::Col => {
            board.board[axis_b][axis_a].candidates_remove(number as usize);
          }
        }
      }
    }
  }
  board
}

fn remove_candidates_from_same_line_other_box(mut board: SudokuBoard, number: u8, n_line: usize, n_box: usize, remove_type: LineType) -> SudokuBoard {
  for axis_b in get_inverse_range_from_box(n_box) {
    match remove_type {
      LineType::Row => {
        board.board[n_line][axis_b].candidates_remove(number as usize);
      }
      LineType::Col => {
        board.board[axis_b][n_line].candidates_remove(number as usize);
      }
    }
  }
  board
}