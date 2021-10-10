use crate::sudoku_types::{SudokuCell, SudokuCellType};

pub fn get_range_from_pos(pos: usize) -> [usize; 3] {
  if pos < 3 {
    return [0, 1, 2];
  }
  if pos < 6 {
    return [3, 4, 5];
  }
  if pos < 9 {
    return [6, 7, 8];
  }
  panic!("get_range_from_pos cannot calculate for {}", pos)
}

pub fn get_range_of_box(n_box: usize) -> ([usize; 3], [usize; 3]) {
  return get_box_range_from_pos(n_box / 3, n_box % 3);
}

pub fn get_box_range_from_pos(x: usize, y: usize) -> ([usize; 3], [usize; 3]) {
  return (get_range_from_pos(x), get_range_from_pos(y));
}

pub fn get_inverse_range_from_pos(pos: usize) -> [usize; 6] {
  if pos < 3 {
    return [3, 4, 5, 6, 7, 8];
  }
  if pos < 6 {
    return [0, 1, 2, 6, 7, 8];
  }
  if pos < 9 {
    return [0, 1, 2, 3, 4, 5];
  }
  panic!("get_range_from_pos cannot calculate for {}", pos)
}

pub fn get_candidates_from_cell(cell: SudokuCell) -> Vec<usize> {
  let mut vector = vec![];
  for i in 0..9 {
    if cell.candidates[i] {
      vector.push(i + 1);
    }
  }
  return vector;
}

pub fn contains_candidates_in_list(cell: SudokuCell, vector: &Vec<usize>) -> bool {
  let mut response = true;
  if cell.cell_type != SudokuCellType::Empty {
    return false;
  }
  for i in 0..9 {
    if cell.candidates[i] {
      if !vector.contains(&(i + 1)) {
        response = false;
      }
    }
  }
  return response;
}