use crate::board_util::{contains_candidates_in_list, get_box_range_from_pos, get_candidates_from_cell, get_range_of_box};
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuCellType};

pub(super) fn naked_tuple(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let hits = 0;
  for x in 0..9 {
    for y in 0..9 {
      let candidates = get_candidates_from_cell(board.get_cell(x, y));
      let size = candidates.len();
      if size > 1 {
        let mut occurrences_box: u8 = 0;
        let mut occurrences_row: u8 = 0;
        let mut occurrences_col: u8 = 0;
        let box_x = (x) / 3;
        let box_y = (y) % 3;
        // For box
        for n_box in 0..9 {
          let box_range = get_range_of_box(n_box);
          for x in box_range.0 {
            for y in box_range.1 {
              let is_match = contains_candidates_in_list(board.get_cell(x, y), &vector);
              if is_match {

                occurrences_box += 1;
              }
            }
          }
          if occurrences_box == size as u8 {
            // Limpiar por el resto del quarto de las que no hacen match:
            clean_candidates_box_naked(board, );
          }
        }
      }
    }
  }
  (board, hits)
}

fn clean_candidates_box_naked(mut board: SudokuBoard, base_x: usize, base_y: usize) -> (SudokuBoard, isize) {
  let mut hits = 0;
  let box_range = get_box_range_from_pos(base_x, base_y);
  for x in box_range.0 {
    for y in box_range.1 {
      // ToDo: Check if need to repeat
      let candidates: Vec<usize> = get_candidates_from_cell(board.get_cell(base_x, base_y));
      let is_match = contains_candidates_in_list(board.get_cell(x, y), &candidates);
      if !is_match {
        // tenemos que eliminar todos lo que no estan presentes
        let to_delete_candidates = get_candidates_from_cell(board.get_cell(x, y));
        for n in to_delete_candidates {
          if board.board[x][y].candidates[n -1] {
          hits += 1;
          }
          board.board[x][y].candidates[n -1] = false;
        }
      }
    }
  }
  return (board, hits)
}

pub(super) fn naked_single(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let mut hits: isize = 0;
  for x in 0..9 {
    for y in 0..9 {
      let cell = board.get_cell(x, y);
      if cell.is_empty() {
        let candidates = get_candidates_from_cell(cell);
        if candidates.len() == 1 {
          board.set_guess(*candidates.get(0).unwrap(), x, y);
          hits += 1;
        }
      }
    }
  }
  return (board, hits);
}

// ToDo: End single
// ToDo: naked pair, triples and Quads
// En una unidad se encuentra un numero
// En una columna, fila o caja se encuentran para las veces que aparece conjuntos de n tuplas
// No tienen por que aparecer, pero no puede haber ninguno mas
