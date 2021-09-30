use std::ops::Range;
use crate::sudoku_process::{SudokuCell, SudokuCellType};

pub fn get_numbers_from_cell(cell: SudokuCell) -> Vec<usize> {
  let mut vector = vec![];
  for i in 0..9 {
    if cell.possibles[i] {
      vector.push(i + 1);
    }
  }
  return vector;
}

pub fn check_if_occurrence(cell: SudokuCell, vector: &Vec<usize>) -> bool {
  let mut response = true;
  if cell.cell_type != SudokuCellType::Empty{
    return false;
  }
  for i in 0..9 {
    if cell.possibles[i] {
      if !vector.contains(&(i + 1)) {
        response = false;
      }
    }
  }
  return response;
}

pub fn check_same_subsegment(a: usize, b: usize) -> bool{
  if a < 3 {
    return b < 3;
  }
  if a < 6 {
    return b < 6
  }
  if a < 9 {
    return b < 9
  }
  return false;
}

pub fn get_range_from_n(a: usize) -> Vec<usize> {
  if a < 3 {
    if a == 0 {
      return (3..9).collect();
    }
  }
  if a < 6 {
    let mut response: Vec<usize> = [(0..3).collect::<Vec<_>>(), (6..9).collect::<Vec<_>>()].concat();
    return response;
  }
  if a < 9 {
    return (0..6).collect();
  }
  return (0..0).collect();
}