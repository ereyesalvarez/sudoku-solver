// ToDo: Hidden single, pair, triples, quad
// En una fila, columna o caja hay un conjunto de numeros que solo aparecen n veces, siendo n el numero de repeticiones
// para que el sistema funcione tienen que estar todos o al menos uno.
// p. e. 1, 5, 6 - 5, 6, 9 - 3, 6  aunque el 5 no aparezca en las tres ocasiones
// si ni el 5 ni el 6 estan en ninguna otra casilla del conjunto podemos borrar el resto de numeros de las celdas en las que aparecen
// El orden de probabilidad es 1, 2, 3, 4
// En el caso de Single tambien se conoce como pinned y simplemente setea


use crate::board_util::{contains_hidden_candidates_in_list,  get_x_y_from_box_and_pos};
use crate::sudoku_types::{LineType, SudokuBoard};

pub(super) fn hidden_tuple(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let mut hits = 0;
  for a in 1..10 {
    for b in (a + 1) ..10 {
      for c in (b + 1)..11 {
        let  (aux_board, aux_hit) = if c == 10 {
          hidden_tuple_check(board, vec![a, b])
        } else {
          hidden_tuple_check(board, vec![a, b, c])
        };
        board = aux_board;
        hits += aux_hit;
      }
    }
  }
  (board, hits)
}

fn hidden_tuple_check(mut board: SudokuBoard, candidates: Vec<usize>) -> (SudokuBoard, isize) {
  let mut hits = 0;
  let len = candidates.len();
  for axis_a in 0..9 {
    let mut occurrences_row: u8 = 0;
    let mut occurrences_col: u8 = 0;
    let mut occurrences_box: u8 = 0;
    for axis_b in 0..9 {
      if contains_hidden_candidates_in_list(board.get_cell(axis_a, axis_b), &candidates) {
        occurrences_row += 1;
      }
      if contains_hidden_candidates_in_list(board.get_cell(axis_b, axis_a), &candidates) {
        occurrences_col += 1;
      }
      if contains_hidden_candidates_in_list(board.get_cell_box(axis_a, axis_b), &candidates) {
        occurrences_box += 1;
      }
    }
    if occurrences_row == len as u8 {
      let  (aux_board, aux_hit) = clean_candidates_line_hidden(board, &candidates, axis_a, LineType::Row);
      board = aux_board;
      hits += aux_hit;
    }
  }
  (board, hits)
}

fn clean_candidates_line_hidden(mut board: SudokuBoard, candidates: &Vec<usize>, axis: usize, line_type: LineType) -> (SudokuBoard, isize) {
  let  hits = 0;
  for axis_b in 0..9 {
    let (x, y) = match line_type {
      LineType::Row =>  (axis, axis_b),
      LineType::Col =>  (axis_b, axis)
    };
    if contains_hidden_candidates_in_list(board.get_cell(x, y), &candidates) {
      for n in 1..10 {
        if !candidates.contains(&n) {
          board.board[x][y].candidates_remove(n);
        }
      }
    }
  }
  (board, hits)
}
fn clean_candidates_box_hidden(mut board: SudokuBoard, candidates: &Vec<usize>, n_box: usize) -> (SudokuBoard, isize) {
  let  hits = 0;
  for n_position in 0..9 {
    if contains_hidden_candidates_in_list(board.get_cell_box(n_box, n_position), &candidates) {
      for n in 1..10 {
        if !candidates.contains(&n) {
          let (x, y) = get_x_y_from_box_and_pos(n_box, n_position);
          board.board[x][y].candidates_remove(n);
        }
      }
    }
  }
  (board, hits)
}