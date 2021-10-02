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
  clean_hide_pair(board);
}

fn is_hidden_pais_candidate(cell: SudokuCell, a: usize, b:usize) -> (bool, bool){
  // a and b are positions on array not numbers;
  if cell.is_empty() {
    let a_present = cell.possibles[a];
    let b_present = cell.possibles[b];
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

fn clean_hide_pair(board: &mut [[SudokuCell; 9]; 9]) {
  for x in 0..10 {
    let mut pairs: HashMap<PairTuple, usize> = HashMap::new();
    for a in 0..10 {
      // Check if a can b picked
      for b in a..10 {
        let mut vec = Vec::new();
        let mut count = 0;
        let mut abort = false;
        for y in 0..10 {
          let (is_match, partial_abort) = is_hidden_pais_candidate(board[x][y], a, b);
          if is_match {
            count += 1;
            vec.push(PairTuple { x, y })
          }
          if partial_abort && !abort{
            abort = true;
          }
        }
        if !abort && count == 2 {
          for tuple in vec {
            for l in 0..9 {
              if l != a && l != b {
                board[tuple.x][tuple.y].possibles[l] = false;
              }
            }
          }
        }
      }
    }
  }
}