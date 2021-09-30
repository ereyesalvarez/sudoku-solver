use crate::sudoku_util::calculate_quarter;

#[derive(Copy, Clone)]
pub struct SudokuPossibles {
  pub row: [[bool; 9]; 9],
  pub col: [[bool; 9]; 9],
  pub quarter: [[bool; 9]; 9],
}

pub fn reset_possibles() -> SudokuPossibles {
  return SudokuPossibles {
    row: [[true; 9]; 9],
    col: [[true; 9]; 9],
    quarter: [[true; 9]; 9],
  };
}

pub fn remove_from_possibles(x: usize, y: usize, n: usize, input: &mut SudokuPossibles) {
  rm_possible_double(x, n, &mut input.row);
  rm_possible_double(y, n, &mut input.col);
  rm_possible_double(calculate_quarter(x, y), n, &mut input.quarter);
}

fn rm_possible_double(i: usize, n: usize, array: &mut [[bool; 9]; 9]) {
  array[i][n-1] = false;
}
