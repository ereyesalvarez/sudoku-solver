use crate::sudoku_types::{SudokuCell, SudokuCellType};

pub(crate) fn get_x_y_from_box_and_pos(n_box: usize, position: usize) -> (usize, usize) {
  // 0 - 8
  let x = (n_box % 3 * 3) + position % 3;
  let y = (n_box / 3 * 3) + position / 3;
  (x, y)
}

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


pub fn get_inverse_range_from_box(n_box: usize) -> [usize; 6] {
  return match n_box {
    0 => return [3, 4, 5, 6, 7, 8],
    1 => return [0, 1, 2, 6, 7, 8],
    2 => return [0, 1, 2, 3, 4, 5],
    _ =>   panic!("get_inverse_range_from_box cannot calculate for {}", n_box)
  }
}

pub fn get_inverse_range_from_pos(pos: usize) -> [usize; 6] {
  if pos < 3 {
    return get_inverse_range_from_box(0);
  }
  if pos < 6 {
    return get_inverse_range_from_box(1);
  }
  if pos < 9 {
    return get_inverse_range_from_box(2);
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

/// recibe una celda y un vector si la celda contiene solo los numeros del vector devuelve true
/// Si presente en celda como candidato entonces tiene que estar en el vector
pub fn contains_candidates_in_list(cell: SudokuCell, vector: &Vec<usize>) -> bool {
  let mut response = true;
  if cell.is_filled() {
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

/// recibe una celda y un vector si la celda contiene todos los numeros del vector true
pub fn contains_hidden_candidates_in_list(cell: SudokuCell, vector: &Vec<usize>) -> bool {
  let mut response = false;
  let mut count = 0;
  if cell.is_filled() {
    return false;
  }
  for i in vector {
    if cell.candidates[i - 1] {
      count += 1;
    }
  }
  if count > vector.len() {
    response = true;
  }
  response
}
