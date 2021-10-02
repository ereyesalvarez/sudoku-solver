use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::sudoku_resolve::utils_clear::remove_possible_same_axis_other_q;
use crate::sudoku_types::{PairTuple, SudokuCell};

/**
Clean with clean pair...
Locate n-tuples of numbers that are only in n positions of the row/column/quarter
If locate one then just remove al possibles from the row/col.

 */
pub fn clean_hide(board: &mut [[SudokuCell; 9]; 9]) {
  // Necesitamos almacenar las asociaciones de numeros y las veces que aparecen p.e. a,b: x,y
  // El numero maximo deberia ser 8 ya que 9 no tiene sentido.
  // Incluso con 8 coincidencias tendriamos uno revelado asi que el maximo es 7
  hide_mono(board); // toDo: need?
  clean_hide_pair(board);
  // clean_hide_pair_col(board);
}

fn is_hidden_pais_candidate(cell: SudokuCell, a: usize, b: usize) -> (bool, bool) {
  // a and b are positions on array not numbers;
  if cell.is_empty() {
    let a_present = cell.is_candidate_present_pos(a);
    let b_present = cell.is_candidate_present_pos(b);
    if a_present && b_present {
      // estan los dos presentes
      return (true, false);
    }
    if (a_present || b_present) && a_present != b_present {
      // esta presente uno sin el otro
      return (false, true);
    }
  }
  return (false, false);
}

fn hide_mono(board: &mut [[SudokuCell; 9]; 9]) {
  for axis_a in 0..9 {
    for n in 0..9 {
      let mut occasions_row = 0;
      let mut occasions_col = 0;
      let mut row_pos = PairTuple { x: 99, y: 99 };
      let mut col_pos = PairTuple { x: 99, y: 99 };
      for axis_b in 0..9 {
        if board[axis_a][axis_b].is_candidate_present_pos(n) {
          // println!("row: n {} {} {}: {}", n, axis_a, axis_b, occasions_row);
          occasions_row += 1;
          row_pos = PairTuple { x: axis_a, y: axis_b };
        }
        if board[axis_b][axis_a].is_candidate_present_pos(n) {
          // println!("col: n {} {} {}: {}", n, axis_b, axis_a, occasions_col);
          occasions_col += 1;
          col_pos = PairTuple { x: axis_b, y: axis_a };
        }
      }
      if occasions_row == 1 {
        board[row_pos.x][row_pos.y].candidates_clean_except(n);
      }
      if occasions_col == 1 {
        board[col_pos.x][col_pos.y].candidates_clean_except(n);
      }
    }
  }
}

fn clean_hide_pair(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..9 {
    for a in 0..9 {
      // Check if a can b picked
      for b in a + 1 ..9 {
        let mut vec = Vec::new();
        let mut count = 0;
        let mut abort = false;
        for y in 0..9 {
          let (is_match, partial_abort) = is_hidden_pais_candidate(board[x][y], a, b);
          if is_match {
            count += 1;
            vec.push(PairTuple { x, y })
          }
          if partial_abort && !abort {
            abort = true;
          }
        }
        if !abort && count == 2 {
          for tuple in vec {
            board[tuple.x][tuple.y].candidates_clean_except_two(a, b);
          }
        }
      }
    }
  }
}